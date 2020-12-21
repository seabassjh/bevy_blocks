//use super::voxel_texturing::{FRAGMENT_SHADER, VERTEX_SHADER};
use building_blocks::core::prelude::*;
use building_blocks::mesh::*;
use building_blocks::storage::{prelude::*, IsEmpty};
use noise::{MultiFractal, NoiseFn, RidgedMulti, Seedable};
use rand::Rng;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages},
        texture::AddressMode,
    },
    tasks::{ComputeTaskPool, TaskPool},
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
enum Terrain {
    Natural,
    Debug,
}

const SEA_LEVEL: f64 = 10.0;
const TERRAIN_Y_SCALE: f64 = 0.2;

impl Terrain {
    fn get_voxels(&self) -> Array3<Voxel> {
        match self {
            Terrain::Natural => {
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
                        let vox_material = rng.gen_range(1, 5) as VoxelMaterial;
                        voxels.fill_extent(&level, Voxel(vox_material));
                    }
                }

                voxels
            },
            Terrain::Debug => {
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
                            Extent3i::from_min_and_shape(PointN([x, 0, z]), PointN([1, 1, 1]));
                        //let vox_material = rng.gen_range(1, 5) as VoxelMaterial;
                        let vox_material = 1 as VoxelMaterial;
                        voxels.fill_extent(&level, Voxel(vox_material));
                    }
                }

                let debug_blocks_0 =
                    Extent3i::from_min_and_shape(PointN([5, 2, 5]), PointN([1, 1, 1]));
                let debug_blocks_1 =
                    Extent3i::from_min_and_shape(PointN([7, 2, 5]), PointN([1, 1, 1]));
                let debug_blocks_2 =
                    Extent3i::from_min_and_shape(PointN([7, 3, 6]), PointN([1, 1, 1]));
                let debug_blocks_3 =
                    Extent3i::from_min_and_shape(PointN([9, 2, 5]), PointN([1, 1, 1]));
                let debug_blocks_4 =
                    Extent3i::from_min_and_shape(PointN([9, 3, 6]), PointN([1, 1, 1]));
                let debug_blocks_5 =
                    Extent3i::from_min_and_shape(PointN([10, 3, 6]), PointN([1, 1, 1]));
                let debug_blocks_6 =
                    Extent3i::from_min_and_shape(PointN([12, 2, 5]), PointN([1, 1, 1]));
                let debug_blocks_7 =
                    Extent3i::from_min_and_shape(PointN([12, 3, 6]), PointN([1, 1, 1]));
                let debug_blocks_8 =
                    Extent3i::from_min_and_shape(PointN([13, 3, 6]), PointN([1, 1, 1]));
                let debug_blocks_9 =
                    Extent3i::from_min_and_shape(PointN([13, 3, 5]), PointN([1, 1, 1]));
                voxels.fill_extent(&debug_blocks_0, Voxel(1));
                voxels.fill_extent(&debug_blocks_1, Voxel(1));
                voxels.fill_extent(&debug_blocks_2, Voxel(1));
                voxels.fill_extent(&debug_blocks_3, Voxel(1));
                voxels.fill_extent(&debug_blocks_4, Voxel(1));
                voxels.fill_extent(&debug_blocks_5, Voxel(1));
                voxels.fill_extent(&debug_blocks_6, Voxel(1));
                voxels.fill_extent(&debug_blocks_7, Voxel(1));
                voxels.fill_extent(&debug_blocks_8, Voxel(1));
                voxels.fill_extent(&debug_blocks_9, Voxel(1));
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

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "620f651b-adbe-464b-b740-ba0e547282ba"]
pub struct MyMaterial {
    pub albedo: Color,
    pub albedo_texture: Option<Handle<Texture>>,
    pub custom_val: f32,
    #[render_resources(ignore)]
    pub shaded: bool,
}

const FRAGMENT_SHADER: &str = "../assets/shaders/voxel.frag";
const VERTEX_SHADER: &str = "../assets/shaders/voxel.vert";

pub fn setup_voxel_generator_system(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server.watch_for_changes().unwrap();
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>(VERTEX_SHADER),
        fragment: Some(asset_server.load::<Shader, _>(FRAGMENT_SHADER)),
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
        custom_val: 0.0,
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
        let chunk_meshes = generate_chunk_meshes(Terrain::Natural, &pool.0);
        for mesh in chunk_meshes.into_iter() {
            if let Some(mesh) = mesh {
                if mesh.pos_norm_tex_mesh.is_empty() {
                    continue;
                }

                state.chunk_mesh_entities.push(create_mesh_entity(
                    mesh,
                    commands,
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
        "Vertex_Voxel_Material",
        VertexAttributeValues::Float(mesh_data.vert_vox_mat_vals),
    );

    render_mesh.set_attribute(
        "Vertex_AO",
        VertexAttributeValues::Float(mesh_data.vert_ao_vals),
    );

    render_mesh.set_indices(Some(Indices::U32(
        mesh.indices.iter().map(|i| *i as u32).collect(),
    )));

    commands
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
    vert_vox_mat_vals: Vec<f32>,
    vert_ao_vals: Vec<f32>,
}

const CHUNK_SIZE: i32 = 16;

fn generate_chunk_meshes(voxel_generation: Terrain, pool: &TaskPool) -> Vec<Option<ChunkMeshData>> {
    let voxels = voxel_generation.get_voxels();

    // Chunk up the voxels just to show that meshing across chunks is consistent.
    let chunk_shape = PointN([CHUNK_SIZE; 3]);
    let ambient_value = Voxel(0);

    let builder = ChunkMapBuilder {
        chunk_shape,
        ambient_value,
        default_chunk_metadata: (),
    };
    // Normally we'd keep this map around in a resource, but we don't need to for this specific example. We could also use an
    // Array3 here instead of a ChunkMap3, but we use chunks for educational purposes.
    let mut map = builder.build_with_hash_map_storage();
    copy_extent(voxels.extent(), &voxels, &mut map);

    // Generate the chunk meshes.
    let map_ref = &map;

    pool.scope(|s| {
        for chunk_key in map_ref.storage().keys() {
            s.spawn(async move {
                let padded_chunk_extent = padded_greedy_quads_chunk_extent(
                    &map_ref.indexer.extent_for_chunk_at_key(*chunk_key),
                );

                let mut padded_chunk = Array3::fill(padded_chunk_extent, Voxel(0));
                copy_extent(&padded_chunk_extent, map_ref, &mut padded_chunk);

                // TODO bevy: we could avoid re-allocating the buffers on every call if we had
                // thread-local storage accessible from this task
                let mut buffer = GreedyQuadsBuffer::new(padded_chunk_extent);
                greedy_quads(&padded_chunk, &padded_chunk_extent, &mut buffer);

                let mut vert_vox_mat_vals: Vec<f32> = Vec::new();
                let mut vert_ao_vals: Vec<f32> = Vec::new();

                let mut mesh = PosNormTexMesh::default();
                for group in buffer.quad_groups.iter() {
                    let y_offset: i32 = match group.face.n {
                        PointN([0, 1, 0]) => {
                            if group.face.n_sign > 0 {
                                0
                            } else {
                                -1
                            }
                        }
                        _ => 0,
                    };

                    for (quad, material) in group.quads.iter() {
                        for v in group.face.quad_corners(quad).iter() {
                            let loc: Point3i = PointN([(v.x()) as i32, (v.y()) as i32, (v.z()) as i32]);
                            let top0 = if padded_chunk_extent.contains(&loc) {
                                let vox: Voxel = padded_chunk.get(&loc);
                                !vox.is_empty()
                            } else {
                                false
                            };
                            
                            let v_ao =
                                get_ao_at_vert(*v, &padded_chunk, &padded_chunk_extent) as f32;
                            vert_ao_vals.extend_from_slice(&[v_ao]);
                        }

                        //vert_ao_vals.extend_from_slice(&[vertex_ao() as f32]);
                        //println!("Location: {:?}", c0.in_voxel());
                        //println!("Voxel: {:?}, Location: {:?}", vox, loc);

                        group.face.add_quad_to_pos_norm_tex_mesh(&quad, &mut mesh);
                        let voxel_mat = *material as f32;
                        vert_vox_mat_vals
                            .extend_from_slice(&[voxel_mat, voxel_mat, voxel_mat, voxel_mat]);
                    }
                }

                if mesh.is_empty() {
                    None
                } else {
                    Some(ChunkMeshData {
                        pos_norm_tex_mesh: mesh,
                        vert_vox_mat_vals,
                        vert_ao_vals,
                    })
                }
            })
        }
    })
}

fn get_ao_at_vert(v: Point3f, padded_chunk: &ArrayN<[i32; 3], Voxel>, padded_chunk_extent: &Extent3i) -> i32
{

    let loc: Point3i = PointN([(v.x()) as i32, (v.y()) as i32, (v.z()) as i32]);

    let top0_loc = PointN([loc.x() - 1, loc.y(), loc.z()]);
    let top1_loc: Point3i = PointN([loc.x(), loc.y(), loc.z() - 1]);
    let top2_loc: Point3i = PointN([loc.x(), loc.y(), loc.z()]);
    let top3_loc: Point3i = PointN([loc.x() - 1, loc.y(), loc.z() - 1]);

    let bot0_loc: Point3i = PointN([loc.x() - 1, loc.y() - 1, loc.z()]);
    let bot1_loc: Point3i = PointN([loc.x(), loc.y() - 1, loc.z() - 1]);
    let bot2_loc: Point3i = PointN([loc.x(), loc.y() - 1, loc.z()]);
    let bot3_loc: Point3i = PointN([loc.x() - 1, loc.y() - 1, loc.z() - 1]);

    let top0 = if padded_chunk_extent.contains(&top0_loc) {
        let vox = padded_chunk.get(&top0_loc);
        !vox.is_empty()
    } else {
        false
    };

    let top1 = if padded_chunk_extent.contains(&top1_loc) {
        let vox = padded_chunk.get(&top1_loc);
        !vox.is_empty()
    } else {
        false
    };
    
    let top2 = if padded_chunk_extent.contains(&top2_loc) {
        let vox = padded_chunk.get(&top2_loc);
        !vox.is_empty()
    } else {
        false
    };
    
    let top3 = if padded_chunk_extent.contains(&top3_loc) {
        let vox = padded_chunk.get(&top3_loc);
        !vox.is_empty()
    } else {
        false
    };

    let bot0 = if padded_chunk_extent.contains(&bot0_loc) {
        let vox = padded_chunk.get(&bot0_loc);
        !vox.is_empty()
    } else {
        false
    };

    let bot1 = if padded_chunk_extent.contains(&bot1_loc) {
        let vox = padded_chunk.get(&bot1_loc);
        !vox.is_empty()
    } else {
        false
    };

    let bot2 = if padded_chunk_extent.contains(&bot2_loc) {
        let vox = padded_chunk.get(&bot2_loc);
        !vox.is_empty()
    } else {
        false
    };

    let bot3 = if padded_chunk_extent.contains(&bot3_loc) {
        let vox = padded_chunk.get(&bot3_loc);
        !vox.is_empty()
    } else {
        false
    };

    let (side0, side1, corner) = 
	if !top0 && bot0 {
		(top2, top3, top1)
	} else {
        if !top1 && bot1 {
            (top2, top3, top0)
        } else {
            if !top2 && bot2 {
                (top0, top1, top3)
            } else {
                if !top3 && bot3 {
                    (top0, top1, top2)
                } else {
                    return 0
                }
            }
        }
    };

    if side0 && side1 {
		return 3;
    } else {
        return side0 as i32 + side1 as i32 + corner as i32
    }
}