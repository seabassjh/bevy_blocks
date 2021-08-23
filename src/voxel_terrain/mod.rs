pub mod generator;
mod save_load;

pub mod constants {
    pub const CHUNK_SIZE: i32 = 16;
    pub const MAX_CHUNK_HEIGHT: i32 = 16;
    pub const SEA_LEVEL: f64 = 50.0;
    pub const TERRAIN_Y_SCALE: f64 = 1.0;
    pub const NUM_TEXTURE_LAYERS: u32 = 5;
    pub const VIEW_DISTANCE: i32 = 192;
}
