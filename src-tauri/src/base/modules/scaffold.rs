use azalea::prelude::*;
use azalea::core::position::BlockPos;
use azalea::registry::builtin::ItemKind;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{convert_inventory_slot_to_hotbar_slot, get_block_state, get_bot_physics, take_item, swing_arm};
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldOptions {
  pub mode: String,
  pub delay: Option<u64>,
  pub min_gaze_degree_x: Option<f32>,
  pub max_gaze_degree_x: Option<f32>,
  pub state: bool
}

impl ScaffoldModule {
  fn is_block(kind: ItemKind) -> bool {
    match kind {
      ItemKind::GrassBlock => { return true; },
      ItemKind::Podzol => { return true; },
      ItemKind::Mycelium => { return true; },
      ItemKind::DirtPath => { return true; },
      ItemKind::Dirt => { return true; },
      ItemKind::CoarseDirt => { return true; },
      ItemKind::RootedDirt => { return true; },
      ItemKind::Farmland => { return true; },
      ItemKind::Mud => { return true; },
      ItemKind::Clay => { return true; },
      ItemKind::Sandstone => { return true; },
      ItemKind::RedSandstone => { return true; },
      ItemKind::Ice => { return true; },
      ItemKind::PackedIce => { return true; },
      ItemKind::BlueIce => { return true; },
      ItemKind::SnowBlock => { return true; },
      ItemKind::MossBlock => { return true; },
      ItemKind::PaleMossBlock => { return true; },
      ItemKind::Stone => { return true; },
      ItemKind::Deepslate => { return true; },
      ItemKind::Granite => { return true; },
      ItemKind::Diorite => { return true; },
      ItemKind::Andesite => { return true; },
      ItemKind::Calcite => { return true; },
      ItemKind::Tuff => { return true; },
      ItemKind::DripstoneBlock => { return true; },
      ItemKind::Prismarine => { return true; },
      ItemKind::Obsidian => { return true; },
      ItemKind::CryingObsidian => { return true; },
      ItemKind::Netherrack => { return true; },
      ItemKind::CrimsonNylium => { return true; },
      ItemKind::WarpedNylium => { return true; },
      ItemKind::SoulSoil => { return true; },
      ItemKind::BoneBlock => { return true; },
      ItemKind::Blackstone => { return true; },
      ItemKind::Basalt => { return true; },
      ItemKind::SmoothBasalt => { return true; },
      ItemKind::EndStone => { return true; },
      ItemKind::OakLog => { return true; },
      ItemKind::SpruceLog => { return true; },
      ItemKind::BirchLog => { return true; },
      ItemKind::JungleLog => { return true; },
      ItemKind::AcaciaLog => { return true; },
      ItemKind::DarkOakLog => { return true; },
      ItemKind::MangroveLog => { return true; },
      ItemKind::CherryLog => { return true; },
      ItemKind::PaleOakLog => { return true; },
      ItemKind::MushroomStem => { return true; },
      ItemKind::CrimsonStem => { return true; },
      ItemKind::WarpedStem => { return true; },
      ItemKind::WhiteWool => { return true; },
      ItemKind::LightGrayWool => { return true; },
      ItemKind::GrayWool => { return true; },
      ItemKind::BlackWool => { return true; },
      ItemKind::BrownWool => { return true; },
      ItemKind::RedWool => { return true; },
      ItemKind::OrangeWool => { return true; },
      ItemKind::YellowWool => { return true; },
      ItemKind::LimeWool => { return true; },
      ItemKind::GreenWool => { return true; },
      ItemKind::CyanWool => { return true; },
      ItemKind::LightBlueWool => { return true; },
      ItemKind::BlueWool => { return true; },
      ItemKind::PurpleWool => { return true; },
      ItemKind::MagentaWool => { return true; },
      ItemKind::PinkWool => { return true; },
      ItemKind::WhiteTerracotta => { return true; },
      ItemKind::LightGrayTerracotta => { return true; },
      ItemKind::GrayTerracotta => { return true; },
      ItemKind::BlackTerracotta => { return true; },
      ItemKind::BrownTerracotta => { return true; },
      ItemKind::RedTerracotta => { return true; },
      ItemKind::OrangeTerracotta => { return true; },
      ItemKind::YellowTerracotta => { return true; },
      ItemKind::LimeTerracotta => { return true; },
      ItemKind::GreenTerracotta => { return true; },
      ItemKind::CyanTerracotta => { return true; },
      ItemKind::LightBlueTerracotta => { return true; },
      ItemKind::BlueTerracotta => { return true; },
      ItemKind::PurpleTerracotta => { return true; },
      ItemKind::MagentaTerracotta => { return true; },
      ItemKind::PinkTerracotta => { return true; },
      ItemKind::WhiteConcrete => { return true; },
      ItemKind::LightGrayConcrete => { return true; },
      ItemKind::GrayConcrete => { return true; },
      ItemKind::BlackConcrete => { return true; },
      ItemKind::BrownConcrete => { return true; },
      ItemKind::RedConcrete => { return true; },
      ItemKind::OrangeConcrete => { return true; },
      ItemKind::YellowConcrete => { return true; },
      ItemKind::LimeConcrete => { return true; },
      ItemKind::GreenConcrete => { return true; },
      ItemKind::CyanConcrete => { return true; },
      ItemKind::LightBlueConcrete => { return true; },
      ItemKind::BlueConcrete => { return true; },
      ItemKind::PurpleConcrete => { return true; },
      ItemKind::MagentaConcrete => { return true; },
      ItemKind::PinkConcrete => { return true; },
      _ => {}
    }

    false
  }

  async fn take_block(bot: &Client) -> bool {
    let menu = bot.menu();

    let mut block_slot = None;

    for slot in menu.player_slots_range() {
      if let Some(item) = menu.slot(slot) {
        if !item.is_empty() {
          if Self::is_block(item.kind()) {
            if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot) {
              if bot.selected_hotbar_slot() == hotbar_slot {
                return true;
              }
            }

            block_slot = Some(slot);
          }
        }
      }
    }

    if let Some(slot) = block_slot {
      take_item(bot, slot).await;
      return true;
    }

    false
  }

  fn simulate_inaccuracy(bot: &Client, direction: (f32, f32)) {
    let inaccurate_direction = (direction.0 + randfloat(-0.08, 0.08) as f32, direction.1 + randfloat(-0.08, 0.08) as f32);

    bot.set_direction(inaccurate_direction.0, inaccurate_direction.1);
  }

  fn direct_gaze(bot: &Client, min_x_rot: Option<f32>, max_x_rot: Option<f32>) {
    let direction = bot.direction();

    let min_x = if let Some(rot) = min_x_rot { rot } else { 80.0 } as f64;
    let max_x = if let Some(rot) = max_x_rot { rot } else { 83.0 } as f64;

    bot.set_direction(direction.0, randfloat(min_x, max_x) as f32); 
  }

  async fn noob_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      if !bot.crouching() {
        bot.set_crouching(true);
      }

      if Self::take_block(bot).await {
        Self::direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let block_under = BlockPos::new(position.x.floor() as i32, (position.y - 0.5).floor() as i32 , position.z.floor() as i32);

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {    
          swing_arm(bot);

          bot.start_use_item();

          sleep(Duration::from_millis(randuint(100, 200))).await;

          Self::simulate_inaccuracy(bot, bot.direction());

          sleep(Duration::from_millis(randuint(100, 150))).await;
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }    
  }

  async fn ninja_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      if Self::take_block(bot).await { 
        Self::direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let pos = bot.position();
        let block_under = BlockPos::new(pos.x.floor() as i32, (pos.y - 0.5).floor() as i32 , pos.z.floor() as i32);

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {
          bot.set_crouching(true);

          sleep(Duration::from_millis(50)).await;
                        
          swing_arm(bot);

          bot.start_use_item();

          sleep(Duration::from_millis(randuint(50, 100))).await;

          Self::simulate_inaccuracy(bot, bot.direction());

          sleep(Duration::from_millis(50)).await;

          bot.set_crouching(false);
        }
      }
              
      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }    
  }

  async fn god_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      if Self::take_block(bot).await { 
        Self::direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let block_under = BlockPos::new(position.x.floor() as i32, (position.y - 0.5).floor() as i32 , position.z.floor() as i32);

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {              
          swing_arm(bot);

          bot.start_use_item(); 

          Self::simulate_inaccuracy(bot, bot.direction());
        }
      }
              
      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }    
  }

  async fn jump_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      if Self::take_block(bot).await { 
        Self::direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let velocity = get_bot_physics(bot).velocity;

        let block_under = BlockPos::new(
          position.x.floor() as i32, 
          (if velocity.y != 0.0 { position.y - 1.0 } else { position.y - 0.5 }).floor() as i32,
          position.z.floor() as i32
        );

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };
                
        if is_air {  
          bot.jump();
                        
          swing_arm(bot);
          
          bot.start_use_item();

          Self::simulate_inaccuracy(bot, bot.direction());
        }  
      }
              
      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }    
  }

  pub async fn enable(bot: &Client, options: ScaffoldOptions) {
    match options.mode.as_str() {
      "noob-bridge" => { Self::noob_bridge_scaffold(bot, options).await; },
      "ninja-bridge" => { Self::ninja_bridge_scaffold(bot, options).await; },
      "god-bridge" => { Self::god_bridge_scaffold(bot, options).await; },
      "jump-bridge" => { Self::jump_bridge_scaffold(bot, options).await; }
      _ => {}
    }
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task("scaffold");
    bot.set_crouching(false);
  }
}