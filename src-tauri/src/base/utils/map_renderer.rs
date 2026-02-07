use azalea::{BlockPos, Vec3};
use azalea::prelude::*;
use azalea::block::BlockState;
use image::{ImageBuffer, ImageFormat, Rgb};
use base64::encode;
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::common::*;


pub static MAP_RENDERER: Lazy<Arc<MapRenderer>> = Lazy::new(|| { Arc::new(MapRenderer::new()) });

pub struct MapRenderer;

impl MapRenderer {
  pub fn new() -> Self {
    Self
  }

  pub fn render(&self, bot: &Client) -> String {
    let pos = bot.position();

    let mut blocks = vec![];

    for x in -512..=512 {
      for z in -512..=512 {
        let vector = Vec3::new(
          pos.x + x as f64,
          pos.y.round(),
          pos.z + z as f64
        );

        let block_pos = BlockPos::from(vector);

        if let Some(state) = get_block_state(bot, block_pos) {
          blocks.push((state, (block_pos.x, block_pos.z)));
        }
      }
    }

    self.create_img(&blocks, pos.x as i32, pos.z as i32)
  }

  fn get_rgb_code(&self, id: u16) -> (u8, u8, u8) {
    match id {
      0 => (224, 255, 255),
      1 => (128, 128, 128),
      86 => (30, 144, 255),
      _ => (0, 0, 0)
    }
  }

  fn create_img(&self, blocks: &Vec<(BlockState, (i32, i32))>, center_x: i32, center_z: i32) -> String {
    let width = 512;
    let height = 512;

    let mut img = ImageBuffer::new(width, height);

    for x in 1..=512 {
      for z in 1..=512 {
        img.put_pixel(x, z, Rgb([224, 255, 255]));
      }
    }

    for (block, (x, z)) in blocks.iter() {
      let rgb = self.get_rgb_code(block.id());

      let img_x = (x - center_x + 256) as u32;
      let img_z = (z - center_z + 256) as u32;

      img.put_pixel(img_x, img_z, Rgb([rgb.0, rgb.1, rgb.2]));
    }

    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);

    let _ = img.write_to(&mut cursor, ImageFormat::Png);

    let base64_code = encode(&bytes);

    base64_code
  }
}