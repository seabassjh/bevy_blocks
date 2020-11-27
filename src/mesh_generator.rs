use building_blocks::core::prelude::*;
use building_blocks::mesh::*;
use building_blocks::storage::{prelude::*, IsEmpty};
use noise::{MultiFractal, NoiseFn, RidgedMulti, Seedable};
use rand::Rng;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
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

pub struct LoadingTexture(Option<Handle<Texture>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cubic {
    Terrace,
}

const SEA_LEVEL: f64 = 10.0;
const TERRAIN_Y_SCALE: f64 = 0.2;

impl Cubic {
    fn get_voxels(&self) -> Array3<CubeVoxel> {
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
                let mut voxels = Array3::fill(extent, CubeVoxel(false));
                for z in 0..40 {
                    for x in 0..40 {
                        let max_y =
                            (noise.get([x as f64, z as f64]) * yscale + yoffset).round() as i32;
                        let level =
                            Extent3i::from_min_and_shape(PointN([x, 0, z]), PointN([1, max_y, 1]));
                        voxels.fill_extent(&level, CubeVoxel(true));
                    }
                }

                voxels
            }
        }
    }
}

#[derive(Clone, Copy)]
struct CubeVoxel(bool);

impl MaterialVoxel for CubeVoxel {
    type Material = u8;

    fn material(&self) -> Self::Material {
        1 // only 1 material
    }
}

impl IsEmpty for CubeVoxel {
    fn is_empty(&self) -> bool {
        !self.0
    }
}

#[derive(Default)]
pub struct MeshMaterial(pub Handle<StandardMaterial>);

pub fn setup_mesh_generator_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Start loading the texture.
    commands.insert_resource(LoadingTexture(Some(
        asset_server.load("../assets/textures/stone.png"),
    )));
}

pub fn mesh_generator_system(
    mut commands: Commands,
    mut loading_texture: ResMut<LoadingTexture>,
    mut textures: ResMut<Assets<Texture>>,
    pool: Res<ComputeTaskPool>,
    mut state: ResMut<MeshGeneratorState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let new_shape_requested = false;

    if new_shape_requested || state.chunk_mesh_entities.is_empty() {
        // Delete the old meshes.
        for entity in state.chunk_mesh_entities.drain(..) {
            commands.despawn(entity);
        }

        let (texture_handle, texture) = match loading_texture.0.as_ref() {
            Some(handle) => {
                if let Some(texture) = textures.get_mut(handle) {
                    (loading_texture.0.take().unwrap(), texture)
                } else {
                    return;
                }
            }
            None => return,
        };

        texture.sampler.address_mode_u = AddressMode::Repeat;
        texture.sampler.address_mode_v = AddressMode::Repeat;
        texture.sampler.address_mode_w = AddressMode::Repeat;

        let material_handle = materials.add(StandardMaterial {
            albedo_texture: Some(texture_handle.clone()),
            shaded: true,
            ..Default::default()
        });

        // Sample the new shape.
        let chunk_meshes = generate_chunk_meshes_from_cubic(Cubic::Terrace, &pool.0);

        for mesh in chunk_meshes.into_iter() {
            if let Some(mesh) = mesh {
                if mesh.indices.is_empty() {
                    continue;
                }

                state.chunk_mesh_entities.push(create_mesh_entity(
                    mesh,
                    &mut commands,
                    material_handle.clone(),
                    &mut meshes,
                ));
            }
        }
    }
}

const CHUNK_SIZE: i32 = 16;

fn generate_chunk_meshes_from_cubic(cubic: Cubic, pool: &TaskPool) -> Vec<Option<PosNormTexMesh>> {
    let voxels = cubic.get_voxels();

    // Chunk up the voxels just to show that meshing across chunks is consistent.
    let chunk_shape = PointN([CHUNK_SIZE; 3]);
    let ambient_value = CubeVoxel(false);
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

                let mut padded_chunk = Array3::fill(padded_chunk_extent, CubeVoxel(false));
                copy_extent(&padded_chunk_extent, &map_reader, &mut padded_chunk);

                // TODO bevy: we could avoid re-allocating the buffers on every call if we had
                // thread-local storage accessible from this task
                let mut buffer = GreedyQuadsBuffer::new(padded_chunk_extent);
                greedy_quads(&padded_chunk, &padded_chunk_extent, &mut buffer);

                let mut mesh = PosNormTexMesh::default();
                for group in buffer.quad_groups.iter() {
                    for (quad, _material) in group.quads.iter() {
                        group.meta.add_quad_to_pos_norm_tex_mesh(&quad, &mut mesh);
                    }
                }

                if mesh.is_empty() {
                    None
                } else {
                    Some(mesh)
                }
            })
        }
    })
}

fn create_mesh_entity(
    mesh: PosNormTexMesh,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    assert_eq!(mesh.positions.len(), mesh.normals.len());

    let _num_vertices = mesh.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float2(mesh.tex_coords),
    );
    render_mesh.set_indices(Some(Indices::U32(
        mesh.indices.iter().map(|i| *i as u32).collect(),
    )));

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(render_mesh),
            material,
            ..Default::default()
        })
        .current_entity()
        .unwrap()
}
