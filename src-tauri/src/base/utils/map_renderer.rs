use azalea::{BlockPos, Vec3};
use azalea::prelude::*;
use azalea::block::BlockState;
use image::{ImageBuffer, ImageFormat, Rgb};
use base64::encode;
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::common::*;
use crate::emit::{EventType, MapRenderProgressEventPayload, emit_event};


pub static MAP_RENDERER: Lazy<Arc<MapRenderer>> = Lazy::new(|| { Arc::new(MapRenderer::new()) });

pub struct MapRenderer;

impl MapRenderer {
  pub fn new() -> Self {
    Self
  }

  pub fn render(&self, bot: &Client) -> String {
    let pos = bot.position();

    let mut blocks = vec![];

    let mut test = vec![];

    let mut progress = 0;

    for x in -128..=128 {
      for z in -128..=128 {
        progress += 1;

        emit_event(EventType::MapRenderProgress(MapRenderProgressEventPayload {
          nickname: bot.username(),
          progress: progress
        }));

        let mut exist_block_above = false;

        for y in 0..=20 {
          let vector = Vec3::new(
            pos.x + x as f64,
            pos.y + y as f64,
            pos.z + z as f64
          );

          let block_pos = BlockPos::from(vector);

          if let Some(state) = get_block_state(bot, block_pos) {
            if !test.contains(&state.id()) {
              test.push(state.id());
              println!("Block: {:?}", state);
            }

            if !state.is_air() {
              let mut current_y = vector.y;

              loop {
                current_y += 1.0;

                if let Some(s) = get_block_state(bot, BlockPos::from(Vec3::new(vector.x, current_y, vector.z))) {
                  if s.is_air() {
                    current_y -= 1.0;
                    break;
                  }
                }
              }

              let new_block_pos = BlockPos::from(Vec3::new(vector.x, current_y, vector.z));

              if let Some(s) = get_block_state(bot, new_block_pos) {
                blocks.push((s, (block_pos.x, block_pos.z)));
                exist_block_above = true;
                break;
              }
            } else {
              blocks.push((state, (block_pos.x, block_pos.z)));
            }
          }
        } 

        if !exist_block_above {
          for y in 0..=20 {
            let vector = Vec3::new(
              pos.x + x as f64,
              pos.y - y as f64,
              pos.z + z as f64
            );

            let block_pos = BlockPos::from(vector);

            if let Some(state) = get_block_state(bot, block_pos) {
              blocks.push((state, (block_pos.x, block_pos.z)));

              if !state.is_air() {
                break;
              }
            }
          } 
        }
      }
    }

    self.create_img(&blocks, pos.x as i32, pos.z as i32)
  }

  fn get_rgb_code(&self, id: u16) -> (u8, u8, u8) {
    match id {
      0 => (0, 0, 0),
      1 => (128, 128, 128),
      14 => (128, 128, 128),
      2 => (165, 42, 42),
      4 => (192, 192, 192),
      6 => (192, 192, 192),
      124 => (169, 169, 169),
      9 => (0, 128, 0),
      12722 => (0, 128, 0),
      2048 => (0, 128, 0),
      2123 => (0, 128, 0),
      2132 => (0, 128, 0),
      10 => (139, 69, 19),
      133 => (48, 48, 48),
      25111 => (206, 126, 80),
      118 => (240, 230, 140),
      578 => (238, 232, 170),
      90 => (30, 144, 255),
      93 => (30, 144, 255),
      92 => (30, 144, 255),
      91 => (30, 144, 255),
      94 => (30, 144, 255),
      86 => (30, 144, 255),
      87 => (30, 144, 255),
      88 => (30, 144, 255),
      89 => (30, 144, 255),
      2055 => (30, 144, 255),
      2054 => (30, 144, 255),
      3168 => (0, 0, 128),
      21618 => (0, 0, 128),
      131 => (220, 220, 220),
      2104 => (0, 0, 255),
      2096 => (0, 191, 255),
      2107 => (255, 0, 0),
      2099 => (255, 192, 203),
      2094 => (255, 165, 0),
      2095 => (255, 0, 255),
      2102 => (0, 255, 255),
      2098 => (0, 255, 0),
      27645 => (160, 82, 45),
      27655 => (160, 82, 45),
      27659 => (160, 82, 45),
      27652 => (160, 82, 45),
      27646 => (160, 82, 45),
      27654 => (160, 82, 45),
      27647 => (160, 82, 45),
      27657 => (160, 82, 45),
      27649 => (160, 82, 45),
      27648 => (160, 82, 45),
      27644 => (160, 82, 45),
      27658 => (160, 82, 45),
      27653 => (160, 82, 45),
      27656 => (160, 82, 45),
      323 => (0, 100, 0),
      311 => (0, 100, 0),
      315 => (0, 100, 0),
      319 => (0, 100, 0),
      379 => (34, 139, 34),
      375 => (34, 139, 34),
      371 => (34, 139, 34),
      367 => (34, 139, 34),
      383 => (34, 139, 34),
      255 => (0, 100, 0),
      263 => (0, 100, 0),
      259 => (0, 100, 0),
      267 => (0, 100, 0),
      102 => (255, 94, 0),
      563 => (0, 47, 255),
      27553 => (205, 133, 63),
      _ => (0, 0, 0)
    }
  }

  fn create_img(&self, blocks: &Vec<(BlockState, (i32, i32))>, center_x: i32, center_z: i32) -> String {
    let width = 256;
    let height = 256;

    let mut img = ImageBuffer::new(width, height);

    for (block, (x, z)) in blocks.iter() {
      let rgb = self.get_rgb_code(block.id());

      let img_x = (x - center_x + 128) as u32;
      let img_z = (z - center_z + 128) as u32;

      if img_x <= 256 && img_z <= 256 {
        img.put_pixel(img_x, img_z, Rgb([rgb.0, rgb.1, rgb.2]));
      }
    }

    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);

    let _ = img.write_to(&mut cursor, ImageFormat::Png);

    let base64_code = encode(&bytes);

    base64_code
  }
}