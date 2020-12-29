use super::{
    constants::CHUNK_SIZE,
    generator::{Voxel, VoxelMap},
};
use building_blocks::core::prelude::*;
use building_blocks::storage::{compression::Lz4, prelude::*, BincodeCompression};
use fnv::FnvHashMap;
use std::io::{prelude::*, BufReader};
use std::{fs::File, io::Error};

const SAVE_DIR: &str = "./map_save";

pub fn save_chunk_to_file(
    pos: Point3i,
    voxel_map: &VoxelMap,
    extent: Extent3i,
) -> Result<(), Error> {
    let file_name = format!("{}/chunk_{}_{}", SAVE_DIR, pos.x(), pos.z());
    let mut file = File::create(file_name)?;
    let bytes = serialize_chunk(voxel_map, extent);
    file.write_all(&bytes)?;
    Ok(())
}

pub fn load_chunk_from_file(
    pos: Point3i,
    voxel_map: &mut VoxelMap,
    extent: Extent3i,
) -> Result<(), Error> {
    let file_name = format!("{}/chunk_{}_{}", SAVE_DIR, pos.x(), pos.z());
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = Vec::new();
    buf_reader.read_to_end(&mut buf)?;
    deserialize_chunk(buf, extent, voxel_map);
    Ok(())
}

fn serialize_chunk(voxel_map: &VoxelMap, extent: Extent3i) -> Vec<u8> {
    let _extent_padded = extent.padded(1);

    let builder = ChunkMapBuilder {
        chunk_shape: PointN([CHUNK_SIZE; 3]),
        ambient_value: Voxel(0),
        default_chunk_metadata: (),
    };

    let mut map = builder.build_with_hash_map_storage();

    copy_extent(&extent, voxel_map, &mut map);

    let compression = Lz4 { level: 10 };
    let serializable = futures::executor::block_on(SerializableChunkMap::from_chunk_map(
        BincodeCompression::new(compression),
        map,
    ));

    let serialized: Vec<u8> = bincode::serialize(&serializable).unwrap();

    serialized
}

fn deserialize_chunk(serialized: Vec<u8>, extent: Extent3i, dst_map: &mut VoxelMap) {
    let deserialized: SerializableChunkMap<[i32; 3], Voxel, (), Lz4> =
        bincode::deserialize(&serialized).unwrap();
    let map = futures::executor::block_on(deserialized.into_chunk_map(FnvHashMap::default()));
    copy_extent(&extent, &map, dst_map);
}
