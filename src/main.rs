mod consts;

use macroquad::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rayon::prelude::*;

include!("../src/consts.rs");

fn window_conf() -> Conf {
  Conf {
    window_title: "Terrain Generator".to_owned(),
    fullscreen: true,
    ..Default::default()
  }
}
#[macroquad::main(window_conf)]
async fn main() {
  let mut seed = SEED_START;
  let mut map = generate_map(seed);

  let mut zoom = 1.2;
  let mut offset_x = -30.0;
  let mut offset_y = 50.0;
  let mut skew = SKEW_START;
  let mut total_height = HEIGHT_LEVELS as f32;
  let mut z_height = Z_HEIGHT_START;
  let texture: Texture2D = load_texture("public/mori.png").await.unwrap();

  loop {
    if is_key_down(KeyCode::Period) {
      zoom *= 1.05;
      offset_y -= 10.0 / zoom;
      offset_x -= 10.0 / zoom;
    }
    if is_key_down(KeyCode::Comma) {
      zoom *= 0.95;
      offset_y += 10.0 / zoom;
      offset_x += 10.0 / zoom;
    }
    if is_key_down(KeyCode::Q) {
      skew *= 1.02;
      offset_y -= (1.0 / (skew) + skew.sqrt() * 5.0) / (2.0 * skew)
    }
    if is_key_down(KeyCode::E) {
      skew *= 0.98;
      offset_y += (1.0 / (skew) + skew.sqrt() * 5.0) / (2.0 * skew)
    }
    if is_key_down(KeyCode::W) {
      if is_key_down(KeyCode::LeftShift) {
        offset_y += 100.0 / zoom;
      } else {
        offset_y += 10.0 / zoom;
      }
    }
    if is_key_down(KeyCode::S) {
      if is_key_down(KeyCode::LeftShift) {
        offset_y -= 100.0 / zoom;
      } else {
        offset_y -= 10.0 / zoom;
      }
    }
    if is_key_down(KeyCode::A) {
      offset_x += 10.0 / zoom;
    }
    if is_key_down(KeyCode::D) {
      offset_x -= 10.0 / zoom;
    }
    if is_key_down(KeyCode::Key1) {
      total_height *= 0.95;
    }
    if is_key_down(KeyCode::Key2) {
      total_height *= 1.05;
    }
    if is_key_down(KeyCode::Key3) {
      if (total_height > 1.0) {
        total_height *= 0.95;
        z_height *= total_height / (total_height * 0.95);
      }
    }
    if is_key_down(KeyCode::Key4) {
      total_height *= 1.05;
      z_height *= total_height / (total_height * 1.05);
    }
    if is_key_down(KeyCode::Key5) {
      total_height *= 1.05;
      z_height *= total_height / (total_height * 1.05);
    }
    if is_key_down(KeyCode::Key6) {
      total_height *= 1.05;
      z_height *= total_height / (total_height * 1.05);
    }
    if is_key_pressed(KeyCode::Space) {
      seed += 1;
      map = generate_map(seed);
    }

    clear_background(*DEEP_WATER);
    draw_map(&map, zoom, offset_x, offset_y, skew, total_height as usize, &texture, z_height).await;

    next_frame().await;
  }
}

fn generate_map(seed: u32) -> Vec<Vec<f32>> {
  let mut map = vec![vec![0.0_f32; MAP_HEIGHT]; MAP_WIDTH];

  noise(&mut map, seed);
  falloff(&mut map);

  map
}

async fn draw_map(map: &Vec<Vec<f32>>, zoom: f32, mut offset_x: f32, offset_y: f32, skew: f32, height_detail: usize, texture: &Texture2D, z_height: f32) {
  for j in 0..MAP_HEIGHT {
    for i in 0..MAP_WIDTH {
      let height = map[i][j] + ELEVATION;
      let color = map_value_to_color(height);
      let height_level = (height * height_detail as f32) as usize;
      let height_offset = height_level as f32 * z_height * zoom;
      let pixel_size = PIXEL_SIZE as f32 * zoom;
      let x = (i as f32 + offset_x) * pixel_size;
      let y = (j as f32 + offset_y) * pixel_size * skew;

      if (i + j) % 2 == 0 {
        // Top face triangles
        draw_triangle(
          vec2(x - pixel_size, y - height_offset),
          vec2(x, y - skew * pixel_size - height_offset),
          vec2(x + pixel_size, y - height_offset),
          *color,
        );
        draw_triangle(
          vec2(x - pixel_size, y - height_offset),
          vec2(x, y + skew * pixel_size - height_offset),
          vec2(x + pixel_size, y - height_offset),
          *color,
        );

        // Side wall left
        draw_triangle(
          vec2(x - pixel_size, y - height_offset),
          vec2(x - pixel_size, y),
          vec2(x, y + skew * pixel_size),
          Color {
            r: color.r - 0.04,
            g: color.g - 0.04,
            b: color.b - 0.04,
            a: color.a,
          },
        );
        draw_triangle(
          vec2(x - pixel_size, y - height_offset),
          vec2(x, y + skew * pixel_size - height_offset),
          vec2(x, y + skew * pixel_size),
          Color {
            r: color.r - 0.04,
            g: color.g - 0.04,
            b: color.b - 0.04,
            a: color.a,
          },
        );

        // Side wall right
        draw_triangle(
          vec2(x + pixel_size, y - height_offset),
          vec2(x + pixel_size, y),
          vec2(x, y + skew * pixel_size),
          Color {
            r: color.r - 0.08,
            g: color.g - 0.08,
            b: color.b - 0.08,
            a: color.a,
          },
        );
        draw_triangle(
          vec2(x + pixel_size, y - height_offset),
          vec2(x, y + skew * pixel_size - height_offset),
          vec2(x, y + skew * pixel_size),
          Color {
            r: color.r - 0.08,
            g: color.g - 0.08,
            b: color.b - 0.08,
            a: color.a,
          },
        );
      }
    }
  }
  texture_draw(texture, offset_x, offset_y, zoom, skew).await;
  draw_text("wasd: Pan", 10.0, 40.0, 30.0, BLACK);
  draw_text("Q-E: Skew", 10.0, 70.0, 30.0, BLACK);
  draw_text(",-.: Zoom", 10.0, 100.0, 30.0, BLACK);
  draw_text("1-2: Height", 10.0, 130.0, 30.0, BLACK);
  draw_text("3-4: Height detail", 10.0, 160.0, 30.0, BLACK);
}
async fn texture_draw(texture: &Texture2D, offset_x: f32, offset_y: f32, zoom: f32, skew: f32) {
  draw_texture_ex(
    texture,
    (offset_x + TEXTURE_COORDS.0) * PIXEL_SIZE as f32 * zoom,
    (offset_y + TEXTURE_COORDS.1) * PIXEL_SIZE as f32 * skew * zoom,
    WHITE,
    DrawTextureParams {
      dest_size: Some(vec2(TEXTURE_SIZE * PIXEL_SIZE as f32 * zoom, TEXTURE_SIZE * PIXEL_SIZE as f32 * zoom * skew)),
      source: Some(Rect::new(
        0.0, 0.0,
        texture.width(),
        texture.height(),
      )),
      ..Default::default()
    },
  );
}


fn noise(map: &mut Vec<Vec<f32>>, seed: u32) {
  let mut noise_maps = Vec::with_capacity(OCTAVES);

  // Generate multiple noise maps
  for i in 0..OCTAVES {
    let perlin = Perlin::new(seed + i as u32);

    let noise_map = generate_noise_map(&perlin);
    noise_maps.push(noise_map);
  }

  // Blend the noise maps
  for i in 0..MAP_WIDTH {
    for j in 0..MAP_HEIGHT {
      map[i][j] = noise_maps.iter().map(|nm| nm[i][j]).sum::<f32>() / OCTAVES as f32;
    }
  }
}

fn generate_noise_map(perlin: &Perlin) -> Vec<Vec<f32>> {
  let mut map = vec![vec![0.0_f32; MAP_HEIGHT]; MAP_WIDTH];

  map.par_iter_mut().enumerate().for_each(|(i, row)| {
    row.iter_mut().enumerate().for_each(|(j, cell)| {
      let mut amplitude = 1.0;
      let mut frequency = 1.0;
      let mut noise_value = 0.0;

      for (k, octave) in (0..OCTAVES).enumerate() {
        noise_value +=
          perlin.get([i as f64 / SCALE * frequency, j as f64 / SCALE * frequency]) as f32
            * amplitude;
        amplitude *= PERSISTENCE.powf(k as f32);
        frequency *= LACUNARITY;
      }

      *cell = noise_value;
    });
  });

  map
}

fn falloff(map: &mut Vec<Vec<f32>>) {
  let center_x = MAP_WIDTH as f32 / 2.0;
  let center_y = MAP_HEIGHT as f32 / 2.0;

  map.par_iter_mut().enumerate().for_each(|(i, row)| {
    row.iter_mut().enumerate().for_each(|(j, cell)| {
      let distance =
        ((center_y - j as f32).abs().powi(2) + (center_x - i as f32).abs().powi(2)).sqrt();
      *cell -= distance.powf(FALLOFF_POWER) / FALLOFF;
    });
  });
}

fn map_value_to_color(height: f32) -> &'static Color {
  match height {
    ..=-0.1 => DEEP_WATER,
    ..=-0.02 => SHALLOW_WATER,
    ..=0.0 => SHORE,
    ..=0.09 => SAND_1,
    ..=0.11 => SAND_2,
    ..=0.12 => SAND_3,
    ..=0.20 => GRASS_1,
    ..=0.24 => GRASS_2,
    ..=0.28 => GRASS_1,
    ..=0.32 => GRASS_2,
    ..=0.35 => GRASS_1,
    ..=0.42 => GRASS_3,
    ..=0.48 => STONE_2,
    ..=0.56 => STONE_1,
    _ => SNOW,
  }
}