use macroquad::color::Color;

const MAP_WIDTH: usize = 420;
const MAP_HEIGHT: usize = 400;
const PIXEL_SIZE: f64 = 4.0;
const SKEW_START: f32 = 0.5;
const Z_HEIGHT_START: f32 = 2.5;
const HEIGHT_LEVELS: usize = 120;
const ELEVATION: f32 = 0.3;
const TEXTURE_COORDS: (f32, f32) = (250.0, 250.0);
const TEXTURE_SIZE: f32 = 0.3;

//Noise
const SEED_START: u32 = 51;
const SCALE: f64 = 200.0;
const FALLOFF: f32 = 1200.0;
const FALLOFF_POWER: f32 = 1.3;
const OCTAVES: usize = 8;
const LACUNARITY: f64 = 2.4;
const PERSISTENCE: f32 = 0.5;

//Colors
const DEEP_WATER: &'static Color = &Color::new(0.01, 0.24, 0.7, 1.0);
const SHALLOW_WATER: &'static Color = &Color::new(0.04, 0.31, 0.77, 1.0);
const SHORE: &'static Color = &Color::new(0.24, 0.41, 0.87, 1.0);
const SAND_1: &'static Color = &Color::new(0.78, 0.69, 0.5, 1.0);
const SAND_2: &'static Color = &Color::new(0.62, 0.57, 0.39, 1.0);
const SAND_3: &'static Color = &Color::new(0.48, 0.43, 0.23, 1.0);
const GRASS_1: &'static Color = &Color::new(0.38, 0.5, 0.2, 1.0);
const GRASS_2: &'static Color = &Color::new(0.47, 0.61, 0.32, 1.0);
const GRASS_3: &'static Color = &Color::new(0.23, 0.39, 0.08, 1.0);
const STONE_1: &'static Color = &Color::new(0.56, 0.56, 0.49, 1.0);
const STONE_2: &'static Color = &Color::new(0.63, 0.63, 0.57, 1.0);
const SNOW: &'static Color = &Color::new(0.95, 0.95, 0.95, 1.0);