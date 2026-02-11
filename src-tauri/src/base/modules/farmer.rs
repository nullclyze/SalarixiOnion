use azalea::core::position::BlockPos;
use azalea::registry::builtin::{BlockKind, ItemKind};
use azalea::{prelude::*, Vec3};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_block_state, get_inventory_menu, take_item};
use crate::tools::*;

#[derive(Debug)]
struct Tool {
  slot: Option<usize>,
  priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmerOptions {
  pub mode: String,
  pub delay: Option<u64>,
  pub state: bool,
}

impl FarmerModule {
  pub fn new() -> Self {
    Self
  }

  async fn auto_tool(&self, bot: &Client) {
    let mut tools = vec![];

    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() {
          match item.kind() {
            ItemKind::WoodenHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 0,
              });
            }
            ItemKind::GoldenHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 1,
              });
            }
            ItemKind::StoneHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 2,
              });
            }
            ItemKind::CopperHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 3,
              });
            }
            ItemKind::IronHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 4,
              });
            }
            ItemKind::DiamondHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 5,
              });
            }
            ItemKind::NetheriteHoe => {
              tools.push(Tool {
                slot: Some(slot),
                priority: 6,
              });
            }
            _ => {}
          }
        }
      }
    }

    let mut best_tool = Tool {
      slot: None,
      priority: 0,
    };

    for w in tools {
      if w.priority > best_tool.priority {
        best_tool = w;
      }
    }

    if let Some(slot) = best_tool.slot {
      take_item(bot, slot).await;
    }
  }

  fn this_is_grown_plant(&self, id: u16) -> bool {
    return vec![10464, 5117, 14612, 10472].contains(&id);
  }

  fn this_is_plant(&self, kind: BlockKind) -> bool {
    return vec![
      BlockKind::Potatoes,
      BlockKind::Carrots,
      BlockKind::Wheat,
      BlockKind::Beetroots,
    ]
    .contains(&kind);
  }

  fn this_is_farmland_without_plant(
    &self,
    bot: &Client,
    kind: BlockKind,
    block_pos: BlockPos,
  ) -> bool {
    if kind == BlockKind::Farmland {
      let block_above = BlockPos::new(block_pos.x, block_pos.y + 1, block_pos.z);

      if let Some(state) = get_block_state(bot, block_above) {
        if state.is_air() {
          return true;
        }
      }
    }

    false
  }

  async fn fertilize_plant(&self, bot: &Client, mode: &String) {
    let mut ferilizer_slot = None;

    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() {
          if item.kind() == ItemKind::BoneMeal {
            ferilizer_slot = Some(slot);
            break;
          }
        }
      }
    }

    if let Some(slot) = ferilizer_slot {
      take_item(bot, slot).await;

      for _ in 0..=4 {
        sleep(Duration::from_millis(self.generate_delay(mode))).await;
        bot.start_use_item();
      }
    }
  }

  async fn look_at_block(&self, bot: &Client, block_pos: BlockPos) {
    let center = block_pos.center();

    if randchance(0.3) {
      let sloppy_pos = Vec3::new(
        center.x as f64 + randfloat(-1.3289, 1.3289),
        center.y as f64 + randfloat(-1.3289, 1.3289),
        center.z as f64 + randfloat(-1.3289, 1.3289),
      );

      bot.look_at(sloppy_pos);

      sleep(Duration::from_millis(50)).await;
    }

    bot.look_at(center);
  }

  async fn take_plant(&self, bot: &Client) -> bool {
    let mut plant_slot = None;

    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() && plant_slot.is_none() {
          match item.kind() {
            ItemKind::Potato => plant_slot = Some(slot),
            ItemKind::Carrot => plant_slot = Some(slot),
            ItemKind::BeetrootSeeds => plant_slot = Some(slot),
            ItemKind::WheatSeeds => plant_slot = Some(slot),
            _ => {}
          }
        }
      }
    }

    if let Some(slot) = plant_slot {
      take_item(bot, slot).await;
      return true;
    }

    false
  }

  fn generate_delay(&self, mode: &String) -> u64 {
    if mode.as_str() == "normal" {
      return randuint(100, 150);
    } else if mode.as_str() == "quick" {
      return 50;
    } else {
      return randuint(20, 30);
    }
  }

  async fn plow_block(&self, bot: &Client, block_pos: BlockPos, mode: &String) {
    if let Some(state) = get_block_state(bot, block_pos) {
      self.auto_tool(bot).await;
      sleep(Duration::from_millis(50)).await;

      let kind = BlockKind::from(state);

      if kind == BlockKind::CoarseDirt || kind == BlockKind::RootedDirt {
        bot.start_use_item();
        sleep(Duration::from_millis(self.generate_delay(mode))).await;
        bot.start_use_item();
      } else {
        bot.start_use_item();
      }
    }
  }

  fn block_can_plowed(&self, bot: &Client, kind: BlockKind, block_pos: BlockPos) -> bool {
    if let Some(state) = get_block_state(
      bot,
      BlockPos::new(block_pos.x, block_pos.y + 1, block_pos.z),
    ) {
      return vec![
        BlockKind::Dirt,
        BlockKind::DirtPath,
        BlockKind::RootedDirt,
        BlockKind::CoarseDirt,
      ]
      .contains(&kind)
        && state.is_air();
    }

    false
  }

  async fn interact_with_block(&self, bot: &Client, block_pos: BlockPos, options: &FarmerOptions) {
    let nickname = bot.username();

    if !STATES.get_state(&nickname, "is_eating")
      && !STATES.get_state(&nickname, "is_drinking")
      && STATES.get_state(&nickname, "can_interacting")
      && STATES.get_state(&nickname, "can_looking")
    {
      if let Some(state) = get_block_state(bot, block_pos) {
        STATES.set_state(&nickname, "can_eating", false);
        STATES.set_state(&nickname, "can_drinking", false);
        STATES.set_mutual_states(&nickname, "looking", true);
        STATES.set_mutual_states(&nickname, "interacting", true);

        let kind = BlockKind::from(state);

        if self.this_is_farmland_without_plant(bot, kind, block_pos) {
          self.look_at_block(bot, block_pos).await;

          if options.mode.as_str() != "ultra" {
            sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
          }

          if self.take_plant(bot).await {
            sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
            bot.start_use_item();
            sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
          }
        } else {
          if self.this_is_plant(kind) {
            self.look_at_block(bot, block_pos).await;

            sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;

            if self.this_is_grown_plant(state.id()) {
              self.auto_tool(bot).await;
              sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
              bot.mine(block_pos).await;
            } else {
              self.fertilize_plant(bot, &options.mode).await;

              if let Some(new_state) = get_block_state(bot, block_pos) {
                sleep(Duration::from_millis(50)).await;

                if self.this_is_grown_plant(new_state.id()) {
                  self.auto_tool(bot).await;
                  sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
                  bot.mine(block_pos).await;
                }
              }
            }

            if options.mode.as_str() != "ultra" {
              sleep(Duration::from_millis(self.generate_delay(&options.mode))).await;
            }
          } else {
            if self.block_can_plowed(bot, kind, block_pos) {
              self.look_at_block(bot, block_pos).await;
              self.plow_block(bot, block_pos, &options.mode).await;
            }
          }
        }

        STATES.set_state(&nickname, "can_eating", true);
        STATES.set_state(&nickname, "can_drinking", true);
        STATES.set_mutual_states(&nickname, "looking", false);
        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }

  async fn farmer(&self, bot: &Client, options: &FarmerOptions) {
    loop {
      for y in -1..=1 {
        let pos = bot.position();

        let block_pos = BlockPos::from(Vec3::new(pos.x, pos.y + y as f64, pos.z));

        self.interact_with_block(bot, block_pos, &options).await;
      }

      for x in -1..=1 {
        for y in -1..=1 {
          for z in -1..=1 {
            let pos = bot.position();

            let block_pos = BlockPos::from(Vec3::new(
              pos.x + x as f64,
              pos.y + y as f64,
              pos.z + z as f64,
            ));

            if block_pos != BlockPos::from(Vec3::new(pos.x, pos.y + y as f64, pos.z)) {
              self.interact_with_block(bot, block_pos, &options).await;
            }
          }
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(100))).await;
    }
  }

  pub async fn enable(&self, bot: &Client, options: &FarmerOptions) {
    self.farmer(bot, options).await;
  }

  pub fn stop(&self, nickname: &String) {
    kill_task(&nickname, "farmer");

    STATES.set_state(&nickname, "can_eating", true);
    STATES.set_state(&nickname, "can_drinking", true);
    STATES.set_mutual_states(&nickname, "looking", false);
    STATES.set_mutual_states(&nickname, "interacting", false);
  }
}
