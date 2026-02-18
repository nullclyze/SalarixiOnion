use azalea::block::BlockState;
use azalea::prelude::*;
use azalea::registry::builtin::BlockKind;
use azalea::{BlockPos, Vec3};
use base64::encode;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::prelude::*;
use image::{ImageBuffer, ImageFormat, Rgb};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::common::*;
use crate::common::{randint, randstr, Classes};
use crate::emit::{emit_event, EventType, LogEventPayload, MapRenderProgressEventPayload};

pub static MAP_RENDERER: Lazy<Arc<MapRenderer>> = Lazy::new(|| Arc::new(MapRenderer::new()));

pub struct MapRenderer;

impl MapRenderer {
  pub fn new() -> Self {
    Self
  }

  pub fn render(&self, bot: &Client) -> String {
    let pos = bot.position();

    let mut blocks = vec![];

    let mut progress = 0;

    for x in -100..=100 {
      for z in -100..=100 {
        progress += 1;

        if progress % 3 == 0 {
          emit_event(EventType::MapRenderProgress(
            MapRenderProgressEventPayload {
              nickname: bot.username(),
              progress: progress,
            },
          ));
        }

        let mut exist_block_above = false;

        for y in 0..=20 {
          let vector = Vec3::new(pos.x + x as f64, pos.y + y as f64, pos.z + z as f64);

          let block_pos = BlockPos::from(vector);

          if let Some(state) = get_block_state(bot, block_pos) {
            if !state.is_air() {
              let mut current_y = vector.y;

              loop {
                current_y += 1.0;

                if let Some(s) = get_block_state(
                  bot,
                  BlockPos::from(Vec3::new(vector.x, current_y, vector.z)),
                ) {
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
          for y in 0..=40 {
            if y % 3 == 0 {
              let vector = Vec3::new(pos.x + x as f64, pos.y - y as f64, pos.z + z as f64);

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
    }

    self.generate_img(&blocks, pos.x as i32, pos.z as i32)
  }

  pub fn save_map(&self, nickname: String, full_path: Option<String>, base64_code: String) {
    if let Some(path) = full_path {
      let result = STANDARD.decode(base64_code);

      match result {
        Ok(bytes) => {
          let prefix = randstr(Classes::Numeric, 4);
          let date = Local::now().format("%H%M%S").to_string();
          let filename = format!("map_{}_{}_{}.png", nickname, prefix, date);

          let create = File::create(format!("{}{}", path, filename));

          match create {
            Ok(mut file) => {
              let _ = file.write_all(&bytes);
            }
            Err(err) => {
              emit_event(EventType::Log(LogEventPayload {
                name: "error".to_string(),
                message: format!("Не удалось сохранить карту {}: {}", nickname, err),
              }));
            }
          }
        }
        Err(err) => {
          emit_event(EventType::Log(LogEventPayload {
            name: "error".to_string(),
            message: format!("Не удалось декодировать карту {}: {}", nickname, err),
          }));
        }
      }
    }
  }

  fn get_rgb_code(&self, kind: BlockKind) -> (u8, u8, u8) {
    match kind {
      // Неплотные блоки
      BlockKind::Air => (0, 0, 0),
      BlockKind::CaveAir => (0, 0, 0),
      BlockKind::Water => (30, 144, 255),
      BlockKind::Lava => (255, 94, 0),
      BlockKind::Cobweb => (224, 221, 220),

      // Руды, рудные блоки
      BlockKind::CopperOre => (208, 90, 47),
      BlockKind::CopperBlock => (208, 90, 47),
      BlockKind::RawCopperBlock => (208, 90, 47),
      BlockKind::IronOre => (204, 204, 204),
      BlockKind::IronBlock => (204, 204, 204),
      BlockKind::RawIronBlock => (204, 204, 204),
      BlockKind::GoldOre => (240, 228, 15),
      BlockKind::GoldBlock => (240, 228, 15),
      BlockKind::RawGoldBlock => (240, 228, 15),
      BlockKind::CoalOre => (54, 54, 54),
      BlockKind::CoalBlock => (54, 54, 54),
      BlockKind::RedstoneOre => (240, 0, 0),
      BlockKind::RedstoneBlock => (240, 0, 0),
      BlockKind::LapisOre => (10, 0, 204),
      BlockKind::LapisBlock => (10, 0, 204),
      BlockKind::EmeraldOre => (0, 204, 0),
      BlockKind::EmeraldBlock => (0, 204, 0),
      BlockKind::DiamondOre => (0, 240, 232),
      BlockKind::DiamondBlock => (0, 240, 232),
      BlockKind::DeepslateCopperOre => (208, 90, 47),
      BlockKind::DeepslateIronOre => (204, 204, 204),
      BlockKind::DeepslateGoldOre => (240, 228, 15),
      BlockKind::DeepslateCoalOre => (54, 54, 54),
      BlockKind::DeepslateRedstoneOre => (240, 0, 0),
      BlockKind::DeepslateLapisOre => (10, 0, 204),
      BlockKind::DeepslateEmeraldOre => (0, 204, 0),
      BlockKind::DeepslateDiamondOre => (0, 240, 232),
      BlockKind::NetherQuartzOre => (232, 232, 232),
      BlockKind::NetherGoldOre => (240, 228, 15),
      BlockKind::AncientDebris => (112, 37, 0),

      // Природные блоки
      BlockKind::Stone => (128, 128, 128),
      BlockKind::Cobblestone => (128, 128, 128),
      BlockKind::Granite => (165, 42, 42),
      BlockKind::Diorite => (192, 192, 192),
      BlockKind::Calcite => (192, 192, 192),
      BlockKind::Tuff => (77, 77, 77),
      BlockKind::Gravel => (169, 169, 169),
      BlockKind::Deepslate => (56, 56, 56),
      BlockKind::Dirt => (139, 69, 19),
      BlockKind::CoarseDirt => (139, 69, 19),
      BlockKind::RootedDirt => (139, 69, 19),
      BlockKind::DirtPath => (200, 160, 76),
      BlockKind::GrassBlock => (0, 128, 0),
      BlockKind::Podzol => (128, 21, 0),
      BlockKind::Mycelium => (120, 120, 120),
      BlockKind::Farmland => (160, 82, 45),
      BlockKind::Mud => (35, 52, 49),
      BlockKind::MossBlock => (0, 224, 11),
      BlockKind::MossCarpet => (0, 224, 11),
      BlockKind::PaleMossBlock => (92, 92, 92),
      BlockKind::PaleMossCarpet => (92, 92, 92),
      BlockKind::SnowBlock => (252, 252, 252),
      BlockKind::Snow => (245, 245, 245),
      BlockKind::Sand => (240, 230, 140),
      BlockKind::Sandstone => (238, 232, 170),
      BlockKind::RedSand => (226, 137, 29),
      BlockKind::RedSandstone => (255, 140, 0),
      BlockKind::Obsidian => (16, 7, 54),
      BlockKind::CryingObsidian => (40, 5, 76),
      BlockKind::Clay => (176, 201, 198),
      BlockKind::Ice => (9, 230, 246),
      BlockKind::PackedIce => (9, 230, 246),
      BlockKind::BlueIce => (9, 230, 246),
      BlockKind::Prismarine => (27, 228, 181),
      BlockKind::DripstoneBlock => (142, 72, 11),
      BlockKind::PointedDripstone => (142, 72, 11),
      BlockKind::MagmaBlock => (230, 122, 0),
      BlockKind::Netherrack => (119, 25, 8),
      BlockKind::CrimsonNylium => (133, 28, 10),
      BlockKind::WarpedNylium => (0, 179, 146),
      BlockKind::SoulSand => (66, 38, 20),
      BlockKind::SoulSoil => (58, 27, 8),
      BlockKind::BoneBlock => (217, 217, 217),
      BlockKind::Blackstone => (41, 41, 41),
      BlockKind::Basalt => (54, 54, 54),
      BlockKind::SmoothBasalt => (54, 54, 54),
      BlockKind::EndStone => (168, 164, 103),
      BlockKind::AmethystBlock => (188, 0, 240),
      BlockKind::BuddingAmethyst => (188, 0, 240),
      BlockKind::SmallAmethystBud => (164, 0, 209),
      BlockKind::MediumAmethystBud => (164, 0, 209),
      BlockKind::LargeAmethystBud => (164, 0, 209),
      BlockKind::AmethystCluster => (164, 0, 209),
      BlockKind::HayBlock => (173, 159, 0),
      BlockKind::Sculk => (6, 16, 91),
      BlockKind::SculkVein => (9, 23, 129),

      // Растительность
      BlockKind::TallGrass => (12, 132, 6),
      BlockKind::TallDryGrass => (12, 132, 6),
      BlockKind::ShortGrass => (12, 132, 6),
      BlockKind::ShortDryGrass => (12, 132, 6),
      BlockKind::Bamboo => (0, 199, 7),
      BlockKind::SugarCane => (0, 209, 7),
      BlockKind::Cactus => (0, 184, 6),
      BlockKind::LilyPad => (0, 148, 5),
      BlockKind::Pumpkin => (240, 136, 0),
      BlockKind::Melon => (30, 199, 0),
      BlockKind::Potatoes => (194, 184, 0),
      BlockKind::Carrots => (194, 126, 0),
      BlockKind::Wheat => (178, 194, 0),
      BlockKind::Beetroots => (168, 50, 0),

      // Листья деревьев
      BlockKind::OakLeaves => (25, 148, 0),
      BlockKind::SpruceLeaves => (19, 112, 0),
      BlockKind::BirchLeaves => (53, 146, 71),
      BlockKind::JungleLeaves => (34, 187, 17),
      BlockKind::AcaciaLeaves => (53, 140, 43),
      BlockKind::DarkOakLeaves => (27, 138, 15),
      BlockKind::MangroveLeaves => (16, 118, 5),
      BlockKind::CherryLeaves => (255, 133, 243),
      BlockKind::PaleOakLeaves => (71, 82, 71),
      BlockKind::AzaleaLeaves => (6, 175, 4),
      BlockKind::FloweringAzaleaLeaves => (6, 170, 3),
      BlockKind::BrownMushroomBlock => (189, 141, 97),
      BlockKind::RedMushroomBlock => (215, 60, 60),
      BlockKind::NetherWartBlock => (145, 8, 8),
      BlockKind::WarpedWartBlock => (8, 145, 116),

      // Дерево, строительные блоки
      BlockKind::OakLog => (108, 53, 15),
      BlockKind::SpruceLog => (101, 55, 21),
      BlockKind::BirchLog => (231, 220, 213),
      BlockKind::JungleLog => (122, 66, 26),
      BlockKind::AcaciaLog => (95, 70, 53),
      BlockKind::DarkOakLog => (77, 31, 0),
      BlockKind::MangroveLog => (102, 41, 0),
      BlockKind::MangroveRoots => (87, 35, 0),
      BlockKind::MuddyMangroveRoots => (75, 37, 12),
      BlockKind::CherryLog => (77, 31, 0),
      BlockKind::PaleOakLog => (59, 46, 38),
      BlockKind::MushroomStem => (211, 197, 187),
      BlockKind::CrimsonStem => (115, 7, 7),
      BlockKind::WarpedStem => (9, 134, 115),

      // Функциональные блоки
      BlockKind::WaterCauldron => (30, 144, 255),
      BlockKind::LavaCauldron => (255, 94, 0),
      BlockKind::Cauldron => (47, 79, 79),
      BlockKind::Anvil => (74, 74, 74),
      BlockKind::DamagedAnvil => (74, 74, 74),
      BlockKind::ChippedAnvil => (74, 74, 74),
      BlockKind::Campfire => (215, 95, 25),
      BlockKind::Bell => (240, 200, 0),

      // Механизмы, редстоун блоки
      BlockKind::RedstoneTorch => (240, 0, 0),
      BlockKind::RedstoneWire => (240, 0, 0),
      BlockKind::Tnt => (191, 13, 13),

      // Цветные блоки
      BlockKind::WhiteWool => (255, 255, 255),
      BlockKind::LightGrayWool => (192, 192, 192),
      BlockKind::GrayWool => (128, 128, 128),
      BlockKind::BlackWool => (0, 0, 0),
      BlockKind::BrownWool => (165, 42, 42),
      BlockKind::RedWool => (255, 0, 0),
      BlockKind::OrangeWool => (255, 165, 0),
      BlockKind::YellowWool => (255, 255, 0),
      BlockKind::LimeWool => (0, 255, 0),
      BlockKind::GreenWool => (0, 128, 0),
      BlockKind::CyanWool => (0, 255, 255),
      BlockKind::LightBlueWool => (30, 144, 255),
      BlockKind::BlueWool => (0, 0, 255),
      BlockKind::PurpleWool => (128, 0, 128),
      BlockKind::MagentaWool => (255, 0, 255),
      BlockKind::PinkWool => (255, 192, 203),
      BlockKind::WhiteCarpet => (255, 255, 255),
      BlockKind::LightGrayCarpet => (192, 192, 192),
      BlockKind::GrayCarpet => (128, 128, 128),
      BlockKind::BlackCarpet => (0, 0, 0),
      BlockKind::BrownCarpet => (165, 42, 42),
      BlockKind::RedCarpet => (255, 0, 0),
      BlockKind::OrangeCarpet => (255, 165, 0),
      BlockKind::YellowCarpet => (255, 255, 0),
      BlockKind::LimeCarpet => (0, 255, 0),
      BlockKind::GreenCarpet => (0, 128, 0),
      BlockKind::CyanCarpet => (0, 255, 255),
      BlockKind::LightBlueCarpet => (30, 144, 255),
      BlockKind::BlueCarpet => (0, 0, 255),
      BlockKind::PurpleCarpet => (128, 0, 128),
      BlockKind::MagentaCarpet => (255, 0, 255),
      BlockKind::PinkCarpet => (255, 192, 203),
      BlockKind::WhiteTerracotta => (255, 255, 255),
      BlockKind::LightGrayTerracotta => (192, 192, 192),
      BlockKind::GrayTerracotta => (128, 128, 128),
      BlockKind::BlackTerracotta => (0, 0, 0),
      BlockKind::BrownTerracotta => (165, 42, 42),
      BlockKind::RedTerracotta => (255, 0, 0),
      BlockKind::OrangeTerracotta => (255, 165, 0),
      BlockKind::YellowTerracotta => (255, 255, 0),
      BlockKind::LimeTerracotta => (0, 255, 0),
      BlockKind::GreenTerracotta => (0, 128, 0),
      BlockKind::CyanTerracotta => (0, 255, 255),
      BlockKind::LightBlueTerracotta => (30, 144, 255),
      BlockKind::BlueTerracotta => (0, 0, 255),
      BlockKind::PurpleTerracotta => (128, 0, 128),
      BlockKind::MagentaTerracotta => (255, 0, 255),
      BlockKind::PinkTerracotta => (255, 192, 203),
      BlockKind::WhiteConcrete => (255, 255, 255),
      BlockKind::LightGrayConcrete => (192, 192, 192),
      BlockKind::GrayConcrete => (128, 128, 128),
      BlockKind::BlackConcrete => (0, 0, 0),
      BlockKind::BrownConcrete => (165, 42, 42),
      BlockKind::RedConcrete => (255, 0, 0),
      BlockKind::OrangeConcrete => (255, 165, 0),
      BlockKind::YellowConcrete => (255, 255, 0),
      BlockKind::LimeConcrete => (0, 255, 0),
      BlockKind::GreenConcrete => (0, 128, 0),
      BlockKind::CyanConcrete => (0, 255, 255),
      BlockKind::LightBlueConcrete => (30, 144, 255),
      BlockKind::BlueConcrete => (0, 0, 255),
      BlockKind::PurpleConcrete => (128, 0, 128),
      BlockKind::MagentaConcrete => (255, 0, 255),
      BlockKind::PinkConcrete => (255, 192, 203),
      BlockKind::WhiteStainedGlass => (255, 255, 255),
      BlockKind::LightGrayStainedGlass => (192, 192, 192),
      BlockKind::GrayStainedGlass => (128, 128, 128),
      BlockKind::BlackStainedGlass => (0, 0, 0),
      BlockKind::BrownStainedGlass => (165, 42, 42),
      BlockKind::RedStainedGlass => (255, 0, 0),
      BlockKind::OrangeStainedGlass => (255, 165, 0),
      BlockKind::YellowStainedGlass => (255, 255, 0),
      BlockKind::LimeStainedGlass => (0, 255, 0),
      BlockKind::GreenStainedGlass => (0, 128, 0),
      BlockKind::CyanStainedGlass => (0, 255, 255),
      BlockKind::LightBlueStainedGlass => (30, 144, 255),
      BlockKind::BlueStainedGlass => (0, 0, 255),
      BlockKind::PurpleStainedGlass => (128, 0, 128),
      BlockKind::MagentaStainedGlass => (255, 0, 255),
      BlockKind::PinkStainedGlass => (255, 192, 203),
      BlockKind::WhiteStainedGlassPane => (255, 255, 255),
      BlockKind::LightGrayStainedGlassPane => (192, 192, 192),
      BlockKind::GrayStainedGlassPane => (128, 128, 128),
      BlockKind::BlackStainedGlassPane => (0, 0, 0),
      BlockKind::BrownStainedGlassPane => (165, 42, 42),
      BlockKind::RedStainedGlassPane => (255, 0, 0),
      BlockKind::OrangeStainedGlassPane => (255, 165, 0),
      BlockKind::YellowStainedGlassPane => (255, 255, 0),
      BlockKind::LimeStainedGlassPane => (0, 255, 0),
      BlockKind::GreenStainedGlassPane => (0, 128, 0),
      BlockKind::CyanStainedGlassPane => (0, 255, 255),
      BlockKind::LightBlueStainedGlassPane => (30, 144, 255),
      BlockKind::BlueStainedGlassPane => (0, 0, 255),
      BlockKind::PurpleStainedGlassPane => (128, 0, 128),
      BlockKind::MagentaStainedGlassPane => (255, 0, 255),
      BlockKind::PinkStainedGlassPane => (255, 192, 203),

      _ => (0, 0, 0),
    }
  }

  fn this_is_loose_block(&self, kind: BlockKind) -> bool {
    match kind {
      BlockKind::Air => return true,
      BlockKind::CaveAir => return true,
      _ => return false,
    }
  }

  fn generate_img(
    &self,
    blocks: &Vec<(BlockState, (i32, i32))>,
    center_x: i32,
    center_z: i32,
  ) -> String {
    let width = 200;
    let height = 200;

    let mut img = ImageBuffer::new(width, height);

    for (block, (x, z)) in blocks.iter() {
      let block_kind = BlockKind::from(*block);

      let color = self.get_rgb_code(block_kind);

      let img_x = x - center_x + 100;
      let img_z = z - center_z + 100;

      if img_x >= 0 && img_x < 200 && img_z >= 0 && img_z < 200 {
        if color == (0, 0, 0) && !self.this_is_loose_block(block_kind) {
          let adjacent_pixels = [
            (img_x + 1, img_z),
            (img_x - 1, img_z),
            (img_x, img_z + 1),
            (img_x, img_z - 1),
            (img_x + 1, img_z + 1),
            (img_x - 1, img_z - 1),
            (img_x + 1, img_z - 1),
            (img_x - 1, img_z + 1),
            (img_x + 2, img_z),
            (img_x - 2, img_z),
            (img_x, img_z + 2),
            (img_x, img_z - 2),
          ];

          for adjacent_pixel in adjacent_pixels {
            if adjacent_pixel.0 >= 0
              && adjacent_pixel.0 < 200
              && adjacent_pixel.1 >= 0
              && adjacent_pixel.1 < 200
            {
              if let Some(pixel) =
                img.get_pixel_checked(adjacent_pixel.0 as u32, adjacent_pixel.1 as u32)
              {
                let new_rgb: Rgb<u8> = *pixel;

                if new_rgb.0 != [0, 0, 0] {
                  if new_rgb.0[0] >= 2
                    && new_rgb.0[0] <= 253
                    && new_rgb.0[1] >= 2
                    && new_rgb.0[1] <= 253
                    && new_rgb.0[2] >= 2
                    && new_rgb.0[2] <= 253
                  {
                    let new_color = [
                      (new_rgb.0[0] as i32 + randint(-2, 2)) as u8,
                      (new_rgb.0[1] as i32 + randint(-2, 2)) as u8,
                      (new_rgb.0[2] as i32 + randint(-2, 2)) as u8,
                    ];
                    img.put_pixel(img_x as u32, img_z as u32, Rgb(new_color));
                  } else {
                    img.put_pixel(img_x as u32, img_z as u32, new_rgb);
                  }

                  break;
                }
              }
            }
          }
        } else {
          img.put_pixel(img_x as u32, img_z as u32, Rgb([color.0, color.1, color.2]));
        }
      }
    }

    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);

    let _ = img.write_to(&mut cursor, ImageFormat::Png);

    let base64_code = encode(&bytes);

    base64_code
  }
}
