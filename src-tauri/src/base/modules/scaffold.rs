use azalea::core::position::BlockPos;
use azalea::{prelude::*, Vec3, WalkDirection};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{
  convert_hotbar_slot_to_inventory_slot, convert_inventory_slot_to_hotbar_slot, get_block_state,
  get_bot_physics, get_inventory_menu, get_selected_hotbar_slot, go, take_item,
  this_is_solid_block, SafeClientImpls,
};
use crate::tools::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldOptions {
  pub mode: String,
  pub delay: Option<u64>,
  pub min_gaze_degree_x: Option<f32>,
  pub max_gaze_degree_x: Option<f32>,
  pub state: bool,
}

impl ScaffoldModule {
  pub fn new() -> Self {
    Self
  }

  async fn take_block(&self, bot: &Client) -> bool {
    if let Some(menu) = get_inventory_menu(bot) {
      if let Some(item) = menu.slot(convert_hotbar_slot_to_inventory_slot(
        get_selected_hotbar_slot(bot),
      )) {
        if this_is_solid_block(item.kind()) {
          return true;
        }
      }

      let mut block_slot = None;

      for (slot, item) in menu.slots().iter().enumerate() {
        if this_is_solid_block(item.kind()) {
          for s in 36..=44 {
            if let Some(i) = menu.slot(s) {
              if this_is_solid_block(i.kind()) {
                if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(s) {
                  if get_selected_hotbar_slot(bot) == hotbar_slot {
                    return true;
                  }
                }
              }
            }
          }

          if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot) {
            if get_selected_hotbar_slot(bot) == hotbar_slot {
              return true;
            }
          }

          block_slot = Some(slot);
          break;
        }
      }

      if let Some(slot) = block_slot {
        take_item(bot, slot, true).await;
        return true;
      }
    }

    false
  }

  fn simulate_inaccuracy(&self, bot: &Client, direction: (f32, f32)) {
    let inaccurate_direction = (
      direction.0 + randfloat(-0.08, 0.08) as f32,
      direction.1 + randfloat(-0.08, 0.08) as f32,
    );

    bot.set_direction(inaccurate_direction.0, inaccurate_direction.1);
  }

  fn direct_gaze(&self, bot: &Client, min_x_rot: Option<f32>, max_x_rot: Option<f32>) {
    let direction = bot.direction();

    let min_x = if let Some(rot) = min_x_rot { rot } else { 80.0 } as f64;
    let max_x = if let Some(rot) = max_x_rot { rot } else { 83.0 } as f64;

    bot.set_direction(direction.0, randfloat(min_x, max_x) as f32);
  }

  fn go_back(&self, bot: Client) {
    tokio::spawn(async move {
      loop {
        if !TASKS.get_task_activity(&bot.username(), "scaffold") {
          break;
        }

        go(&bot, WalkDirection::Backward);
        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn noob_bridge_scaffold(&self, bot: &Client, options: &ScaffoldOptions) {
    loop {
      if !bot.crouching() {
        bot.set_crouching(true);
      }

      if self.take_block(bot).await {
        self.direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let block_under = BlockPos::new(
          position.x.floor() as i32,
          (position.y - 0.5).floor() as i32,
          position.z.floor() as i32,
        );

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {
          bot.swing_arm();

          bot.start_use_item();

          sleep(Duration::from_millis(randuint(100, 200))).await;

          self.simulate_inaccuracy(bot, bot.direction());

          sleep(Duration::from_millis(randuint(100, 150))).await;
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }
  }

  async fn ninja_bridge_scaffold(&self, bot: &Client, options: &ScaffoldOptions) {
    loop {
      if self.take_block(bot).await {
        self.direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let pos = bot.position();
        let block_under = BlockPos::new(
          pos.x.floor() as i32,
          (pos.y - 0.5).floor() as i32,
          pos.z.floor() as i32,
        );

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {
          bot.set_crouching(true);

          sleep(Duration::from_millis(50)).await;

          bot.swing_arm();

          bot.start_use_item();

          sleep(Duration::from_millis(randuint(50, 100))).await;

          self.simulate_inaccuracy(bot, bot.direction());

          sleep(Duration::from_millis(50)).await;

          bot.set_crouching(false);
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }
  }

  async fn god_bridge_scaffold(&self, bot: &Client, options: &ScaffoldOptions) {
    loop {
      if self.take_block(bot).await {
        self.direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let block_under = BlockPos::new(
          position.x.floor() as i32,
          (position.y - 0.5).floor() as i32,
          position.z.floor() as i32,
        );

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {
          bot.swing_arm();

          bot.start_use_item();

          self.simulate_inaccuracy(bot, bot.direction());
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }
  }

  async fn jump_bridge_scaffold(&self, bot: &Client, options: &ScaffoldOptions) {
    loop {
      if self.take_block(bot).await {
        self.direct_gaze(bot, options.min_gaze_degree_x, options.max_gaze_degree_x);

        let position = bot.position();
        let velocity = if let Some(physics) = get_bot_physics(bot) {
          physics.velocity
        } else {
          Vec3::ZERO
        };

        let block_under = BlockPos::new(
          position.x.floor() as i32,
          (if velocity.y != 0.0 {
            position.y - 1.0
          } else {
            position.y - 0.5
          })
          .floor() as i32,
          position.z.floor() as i32,
        );

        let is_air = if let Some(state) = get_block_state(bot, block_under) {
          state.is_air()
        } else {
          false
        };

        if is_air {
          bot.jump();

          bot.swing_arm();

          bot.start_use_item();

          self.simulate_inaccuracy(bot, bot.direction());
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(25))).await;
    }
  }

  pub async fn enable(&self, bot: &Client, options: &ScaffoldOptions) {
    self.go_back(bot.clone());

    match options.mode.as_str() {
      "noob-bridge" => {
        self.noob_bridge_scaffold(bot, options).await;
      }
      "ninja-bridge" => {
        self.ninja_bridge_scaffold(bot, options).await;
      }
      "god-bridge" => {
        self.god_bridge_scaffold(bot, options).await;
      }
      "jump-bridge" => {
        self.jump_bridge_scaffold(bot, options).await;
      }
      _ => {}
    }
  }

  pub fn stop(&self, bot: &Client) {
    let nickname = bot.username();

    kill_task(&nickname, "scaffold");

    bot.set_crouching(false);
    bot.walk(WalkDirection::None);

    STATES.set_mutual_states(&nickname, "walking", false);
    STATES.set_mutual_states(&nickname, "looking", false);
    STATES.set_mutual_states(&nickname, "interacting", false);
  }
}
