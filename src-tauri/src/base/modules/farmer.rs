use azalea::{Vec3, prelude::*};
use azalea::core::position::BlockPos;
use azalea::registry::builtin::ItemKind;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_block_state, take_item};
use crate::tools::*;


#[derive(Debug)]
struct Tool {
  slot: Option<usize>,
  priority: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmerOptions {
  pub mode: String,
  pub state: bool
}

impl FarmerModule {
  async fn auto_tool(bot: &Client) {
    let mut tools = vec![];

    let menu = bot.menu();

    for (slot, item) in menu.slots().iter().enumerate() {
      if !item.is_empty() {
        match item.kind() {
          ItemKind::WoodenHoe => { tools.push(Tool { slot: Some(slot), priority: 0 }); },
          ItemKind::GoldenHoe => { tools.push(Tool { slot: Some(slot), priority: 1 }); },
          ItemKind::StoneHoe => { tools.push(Tool { slot: Some(slot), priority: 2 }); },
          ItemKind::CopperHoe => { tools.push(Tool { slot: Some(slot), priority: 3 }); },
          ItemKind::IronHoe => { tools.push(Tool { slot: Some(slot), priority: 4 }); },
          ItemKind::DiamondHoe => { tools.push(Tool { slot: Some(slot), priority: 5 }); },
          ItemKind::NetheriteHoe => { tools.push(Tool { slot: Some(slot), priority: 6 }); },
          _ => {}
        }
      }
    }

    let mut best_tool = Tool { slot: None, priority: 0 };

    for w in tools {
      if w.priority > best_tool.priority {
        best_tool = w;
      }
    }

    if let Some(slot) = best_tool.slot {
      take_item(bot, slot).await;
    }
  }

  fn this_is_grown_plant(id: u16) -> bool {
    return vec![
      10464, 5117, 
      14612, 10472
    ].contains(&id);
  }

  fn this_is_plant(id: u16) -> bool {
    return vec![
      10465, 10457, 14609,
      14610, 10469, 10463,
      5112, 5110, 10464,
      5117, 14612, 10472
    ].contains(&id);
  }

  fn this_is_garden_bed_without_plant(bot: &Client, id: u16, block_pos: BlockPos) -> bool {
    if id == 5125 {
      let block_above = BlockPos::new(
        block_pos.x,
        block_pos.y + 1,
        block_pos.z
      );

      if let Some(state) = get_block_state(bot, block_above) {
        if state.is_air() {
          return true;
        }
      }
    }
    
    false
  }
  
  async fn fertilize_plant(bot: &Client, mode: String) {
    let mut ferilizer_slot = None;

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if !item.is_empty() {
        if item.kind() == ItemKind::BoneMeal {
          ferilizer_slot = Some(slot);
          break;
        }
      }
    }

    if let Some(slot) = ferilizer_slot {
      take_item(bot, slot).await;
      
      for _ in 0..=4 {
        sleep(Duration::from_millis(if mode.as_str() == "normal" { randuint(50, 100) } else { 50 })).await;
        bot.start_use_item();
      }
    }
  }

  async fn take_plant(bot: &Client) -> bool {
    let mut plant_slot = None;

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if !item.is_empty() && plant_slot.is_none() {
        match item.kind() {
          ItemKind::Potato => { plant_slot = Some(slot) },
          ItemKind::Carrot => { plant_slot = Some(slot) },
          ItemKind::BeetrootSeeds => { plant_slot = Some(slot) },
          ItemKind::WheatSeeds => { plant_slot = Some(slot) },
          _ => {}
        }
      }
    }

    if let Some(slot) = plant_slot {
      take_item(bot, slot).await;
      return true;
    }
    
    false
  }

  async fn look_at_block(bot: &Client, block_pos: BlockPos) {
    let center = block_pos.center();

    if randchance(0.3) {
      let sloppy_pos = Vec3::new(
        center.x as f64 + randfloat(-1.3289, 1.3289),
        center.y as f64 + randfloat(-1.3289, 1.3289),
        center.z as f64 + randfloat(-1.3289, 1.3289)
      );

      bot.look_at(sloppy_pos);

      sleep(Duration::from_millis(50)).await;
    }

    bot.look_at(center);
  }

  fn generate_delay(mode: &String) -> u64 {
    if mode.as_str() == "normal" {
      return randuint(100, 150);
    } else {
      return 50;
    }
  }

  async fn farmer(bot: &Client, options: FarmerOptions) {
    loop {
      let pos = bot.position();

      for x in -1..=1 {
        for y in -1..=1 {
          for z in -1..=1 {
            let block_pos = BlockPos::new(
              pos.x.floor() as i32 + x,
              pos.y.floor() as i32 + y,
              pos.z.floor() as i32 + z
            );

            if let Some(state) = get_block_state(bot, block_pos) {
              if Self::this_is_garden_bed_without_plant(bot, state.id(), block_pos) {
                Self::look_at_block(bot, block_pos).await;
                sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;

                if Self::take_plant(bot).await {
                  sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;
                  bot.start_use_item();
                  sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;
                }
              } else {
                if Self::this_is_plant(state.id()) {
                  Self::look_at_block(bot, block_pos).await;

                  sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;

                  if Self::this_is_grown_plant(state.id()) {
                    Self::auto_tool(bot).await;
                    sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;
                    bot.mine(block_pos).await;
                  } else {
                    Self::fertilize_plant(bot, options.mode.clone()).await;
                  }

                  sleep(Duration::from_millis(Self::generate_delay(&options.mode))).await;
                }
              }
            }
          }
        }
      }
    }
  }

  pub async fn enable(bot: &Client, options: FarmerOptions) {
    Self::farmer(bot, options).await;
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task("farmer");
  }
}