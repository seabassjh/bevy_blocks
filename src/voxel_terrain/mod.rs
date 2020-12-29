pub mod generator;
mod save_load;

mod constants {
    pub const CHUNK_SIZE: i32 = 16;
    pub const SEA_LEVEL: f64 = 10.0;
    pub const TERRAIN_Y_SCALE: f64 = 0.2;
    pub const NUM_TEXTURE_LAYERS: u32 = 5;
}
