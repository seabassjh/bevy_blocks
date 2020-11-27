//use super::voxel_texturing::{FRAGMENT_SHADER, VERTEX_SHADER};
use building_blocks::core::prelude::*;
use building_blocks::mesh::*;
use building_blocks::storage::{prelude::*, IsEmpty};
use noise::{MultiFractal, NoiseFn, RidgedMulti, Seedable};
use rand::Rng;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages},
        texture::AddressMode,
    },
    tasks::{ComputeTaskPool, TaskPool},
    type_registry::TypeUuid,
};

pub struct MeshGeneratorState {
    chunk_mesh_entities: Vec<Entity>,
}

impl MeshGeneratorState {
    pub fn new() -> Self {
        Self {
            chunk_mesh_entities: Vec::new(),
        }
    }
}

pub struct VoxelRenderHandles {
    loading_texture: Option<Handle<Texture>>,
    material: Option<Handle<MyMaterial>>,
    pipeline: Handle<PipelineDescriptor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cubic {
    Terrace,
}

const SEA_LEVEL: f64 = 10.0;
const TERRAIN_Y_SCALE: f64 = 0.2;

impl Cubic {
    fn get_voxels(&self) -> Array3<Voxel> {
        match self {
            Cubic::Terrace => {
                let mut rng = rand::thread_rng();
                let rand_seed: u32 = rng.gen();
                let noise = RidgedMulti::new()
                    .set_seed(rand_seed)
                    .set_frequency(0.008)
                    .set_octaves(5);
                let yoffset = SEA_LEVEL;
                let yscale = TERRAIN_Y_SCALE * yoffset;

                let extent =
                    Extent3i::from_min_and_shape(PointN([-20; 3]), PointN([40; 3])).padded(1);
                let mut voxels = Array3::fill(extent, Voxel(0));
                for z in 0..40 {
                    for x in 0..40 {
                        let max_y =
                            (noise.get([x as f64, z as f64]) * yscale + yoffset).round() as i32;
                        let level =
                            Extent3i::from_min_and_shape(PointN([x, 0, z]), PointN([1, max_y, 1]));
                        let vox_material = rng.gen_range(1,5) as VoxelMaterial;
                        voxels.fill_extent(&level, Voxel(vox_material));
                    }
                }

                voxels
            }
        }
    }
}

type VoxelMaterial = u8;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Voxel(VoxelMaterial);

impl Default for Voxel {
    fn default() -> Self {
        Voxel(0)
    }
}

impl IsEmpty for Voxel {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl MaterialVoxel for Voxel {
    type Material = VoxelMaterial;

    fn material(&self) -> Self::Material {
        self.0
    }
}

#[derive(Default)]
pub struct MeshMaterial(pub Handle<StandardMaterial>);

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "620f651b-adbe-464b-b740-ba0e547282ba"]
pub struct MyMaterial {
    pub albedo: Color,
    pub albedo_texture: Option<Handle<Texture>>,
    pub tex_array_val: u32,
    #[render_resources(ignore)]
    pub shaded: bool,
}

const FRAGMENT_SHADER: &str = include_str!("../assets/shaders/voxel.frag");
const VERTEX_SHADER: &str = include_str!("../assets/shaders/voxel.vert");

pub fn setup_voxel_generator_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<MyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("my_material", base::node::MAIN_PASS)
        .unwrap();

    let texture_handle = asset_server.load("../assets/textures/terrain.png");
    // Create a new material
    let material_handle = materials.add(MyMaterial {
        albedo: Color::rgb(1.0, 1.0, 1.0),
        albedo_texture: Some(texture_handle.clone()),
        tex_array_val: 0,
        shaded: true,
    });
    // // Create a new material
    // let material_handle = materials.add(MyMaterial {
    //     color: Color::rgb(0.0, 0.8, 0.0),
    // });
    // Start loading the texture.
    commands.insert_resource(VoxelRenderHandles {
        loading_texture: Some(texture_handle.clone()),
        material: Some(material_handle.clone()),
        pipeline: pipeline_handle,
    });
}

const NUM_BLOCKS: u32 = 4;

pub fn voxel_generator_system(
    commands: &mut Commands,
    mut assets: ResMut<VoxelRenderHandles>,
    mut textures: ResMut<Assets<Texture>>,
    pool: Res<ComputeTaskPool>,
    mut state: ResMut<MeshGeneratorState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut my_materials: ResMut<Assets<MyMaterial>>,
) {
    let new_shape_requested = false;

    if new_shape_requested || state.chunk_mesh_entities.is_empty() {
        // Delete the old meshes.
        for entity in state.chunk_mesh_entities.drain(..) {
            commands.despawn(entity);
        }

        let (texture_handle, texture) = match assets.loading_texture.as_ref() {
            Some(handle) => {
                if let Some(texture) = textures.get_mut(handle) {
                    (assets.loading_texture.take().unwrap(), texture)
                } else {
                    return;
                }
            }
            None => return,
        };

        texture.sampler.address_mode_u = AddressMode::Repeat;
        texture.sampler.address_mode_v = AddressMode::Repeat;
        texture.sampler.address_mode_w = AddressMode::Repeat;

        // Create a new array texture asset from the loaded texture.
        let array_layers = NUM_BLOCKS + 1;
        texture.reinterpret_stacked_2d_as_array(array_layers);

        let (my_material_handle, material) = match assets.material.as_ref() {
            Some(handle) => {
                if let Some(material) = my_materials.get_mut(handle) {
                    (assets.material.take().unwrap(), material)
                } else {
                    return;
                }
            }
            None => return,
        };


        let material_handle = materials.add(StandardMaterial {
            albedo_texture: Some(texture_handle.clone()),
            shaded: true,
            ..Default::default()
        });

        let render_pipelines =
            RenderPipelines::from_pipelines(vec![RenderPipeline::new(assets.pipeline.clone())]);

        // Sample the new shape.
        let chunk_meshes = generate_chunk_meshes_from_cubic(Cubic::Terrace, &pool.0);
        for mesh in chunk_meshes.into_iter() {
            if let Some(mesh) = mesh {
                if mesh.pos_norm_tex_mesh.is_empty() {
                    continue;
                }

                state.chunk_mesh_entities.push(create_mesh_entity(
                    mesh,
                    commands,
                    material_handle.clone(),
                    my_material_handle.clone(),
                    render_pipelines.clone(),
                    &mut meshes,
                ));
            }
        }
    }
}

fn create_mesh_entity(
    mesh_data: ChunkMeshData,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    my_material: Handle<MyMaterial>,
    pipelines: RenderPipelines,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    let mesh = mesh_data.pos_norm_tex_mesh;

    assert_eq!(mesh.positions.len(), mesh.normals.len());

    let _num_vertices = mesh.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float3(mesh.normals),
    );
    render_mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float2(mesh.tex_coords),
    );
    render_mesh.set_attribute(
        "Vertex_Color",
        VertexAttributeValues::Float(mesh_data.vert_tex_arr_vals),
    );
    render_mesh.set_indices(Some(Indices::U32(
        mesh.indices.iter().map(|i| *i as u32).collect(),
    )));

    commands
        // .spawn(PbrComponents {
        //     mesh: meshes.add(render_mesh),
        //     material,
        //     ..Default::default()
        // })
        .spawn(MeshBundle {
            mesh: meshes.add(render_mesh),
            render_pipelines: pipelines,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(my_material)
        .current_entity()
        .unwrap()
}

struct ChunkMeshData {
    pos_norm_tex_mesh: PosNormTexMesh,
    vert_tex_arr_vals: Vec<f32>,
}

const CHUNK_SIZE: i32 = 16;

fn generate_chunk_meshes_from_cubic(cubic: Cubic, pool: &TaskPool) -> Vec<Option<ChunkMeshData>> {
    let voxels = cubic.get_voxels();

    // Chunk up the voxels just to show that meshing across chunks is consistent.
    let chunk_shape = PointN([CHUNK_SIZE; 3]);
    let ambient_value = Voxel(0);
    let default_chunk_meta = ();
    // Normally we'd keep this map around in a resource, but we don't need to for this specific
    // example. We could also use an Array3 here instead of a ChunkMap3, but we use chunks for
    // educational purposes.
    let mut map = ChunkMap3::new(
        chunk_shape,
        ambient_value,
        default_chunk_meta,
        FastLz4 { level: 10 },
    );
    copy_extent(voxels.extent(), &voxels, &mut map);

    // Generate the chunk meshes.
    let map_ref = &map;

    pool.scope(|s| {
        for chunk_key in map_ref.chunk_keys() {
            s.spawn(async move {
                let local_cache = LocalChunkCache3::new();
                let map_reader = ChunkMapReader3::new(map_ref, &local_cache);
                let padded_chunk_extent =
                    padded_greedy_quads_chunk_extent(&map_ref.extent_for_chunk_at_key(chunk_key));

                let mut padded_chunk = Array3::fill(padded_chunk_extent, Voxel(0));
                copy_extent(&padded_chunk_extent, &map_reader, &mut padded_chunk);

                // TODO bevy: we could avoid re-allocating the buffers on every call if we had
                // thread-local storage accessible from this task
                let mut buffer = GreedyQuadsBuffer::new(padded_chunk_extent);
                greedy_quads(&padded_chunk, &padded_chunk_extent, &mut buffer);

                let mut vert_colors: Vec<f32> = Vec::new();

                let mut mesh = PosNormTexMesh::default();
                for group in buffer.quad_groups.iter() {
                    for (quad, material) in group.quads.iter() {
                        group.meta.add_quad_to_pos_norm_tex_mesh(&quad, &mut mesh);
                        let tex_arr_val = *material as f32;
                        vert_colors
                            .extend_from_slice(&[tex_arr_val, tex_arr_val, tex_arr_val, tex_arr_val]);
                    }
                }

                if mesh.is_empty() {
                    None
                } else {
                    Some(ChunkMeshData {
                        pos_norm_tex_mesh: mesh,
                        vert_tex_arr_vals: vert_colors,
                    })
                }
            })
        }
    })
}
