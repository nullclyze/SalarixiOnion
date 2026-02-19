use azalea::container::ContainerHandle;
use azalea::core::position::BlockPos;
use azalea::prelude::*;
use azalea::Vec3;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_block_state, stop_bot_move};
use crate::generators::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerOptions {
  pub target: String,
  pub radius: Option<i32>,
  pub delay: Option<u64>,
  pub state: bool,
}

impl StealerModule {
  pub fn new() -> Self {
    Self
  }

  fn check_block_id(&self, block_id: u16, target: &String) -> bool {
    match target.as_str() {
      "chest" => {
        return vec![3793, 3787, 3805, 3799].contains(&block_id);
      }
      "barrel" => {
        return vec![20547, 20543, 20541, 20549, 20545].contains(&block_id);
      }
      "shulker" => {
        return vec![
          14666, 14672, 14678, 14720, 14756, 14714, 14762, 14744, 14702, 14696, 14708, 14750,
          14684, 14726, 14738, 14732,
        ]
        .contains(&block_id);
      }
      _ => {}
    }

    false
  }

  fn find_nearest_targets(
    &self,
    bot: &Client,
    center: Vec3,
    target: &String,
    radius: i32,
  ) -> Vec<azalea::BlockPos> {
    let mut positions = Vec::new();

    for x in -radius..=radius {
      for y in -radius..=radius {
        for z in -radius..=radius {
          let block_pos = BlockPos::new(
            (center.x as i32 + x) as i32,
            (center.y as i32 + y) as i32,
            (center.z as i32 + z) as i32,
          );

          if let Some(state) = get_block_state(bot, block_pos) {
            if self.check_block_id(state.id(), target) {
              positions.push(block_pos);
            }
          }
        }
      }
    }

    positions
  }

  async fn extract_all_items(&self, bot: &Client, container: &ContainerHandle) {
    if let Some(menu) = container.menu() {
      stop_bot_move(bot);

      let nickname = bot.username();

      STATES.set_state(&nickname, "can_walking", false);
      STATES.set_state(&nickname, "can_sprinting", false);
      STATES.set_state(&nickname, "can_attacking", false);
      STATES.set_state(&nickname, "can_interacting", false);
      STATES.set_state(&nickname, "can_eating", false);
      STATES.set_state(&nickname, "can_drinking", false);

      for slot in 0..=26 {
        if let Some(item) = menu.slot(slot) {
          if !item.is_empty() {
            container.shift_click(slot);
          }
        }
      }

      STATES.set_state(&nickname, "can_walking", true);
      STATES.set_state(&nickname, "can_sprinting", true);
      STATES.set_state(&nickname, "can_attacking", true);
      STATES.set_state(&nickname, "can_interacting", true);
      STATES.set_state(&nickname, "can_eating", true);
      STATES.set_state(&nickname, "can_drinking", true);
    }
  }

  async fn stealing(&self, bot: &Client, options: &StealerOptions) {
    let nickname = bot.username();

    loop {
      let position = bot.position();
      let direction = bot.direction();

      let target_positions =
        self.find_nearest_targets(bot, position, &options.target, options.radius.unwrap_or(5));

      for pos in target_positions {
        if STATES.get_state(&nickname, "can_looking")
          && STATES.get_state(&nickname, "can_interacting")
        {
          STATES.set_mutual_states(&nickname, "looking", true);
          STATES.set_mutual_states(&nickname, "interacting", true);

          bot.look_at(pos.center());

          sleep(Duration::from_millis(randuint(50, 100))).await;

          if let Some(container) = bot.open_container_at(pos).await {
            self.extract_all_items(bot, &container).await;
            container.close();
            sleep(Duration::from_millis(randuint(200, 350))).await;
          }

          STATES.set_mutual_states(&nickname, "looking", false);
          STATES.set_mutual_states(&nickname, "interacting", false);
        }
      }

      if STATES.get_state(&nickname, "can_looking") {
        bot.set_direction(
          direction.0 + randfloat(-2.5, 2.5) as f32,
          direction.1 + randfloat(-2.5, 2.5) as f32,
        );
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(1000))).await;
    }
  }

  pub async fn enable(&self, bot: &Client, options: &StealerOptions) {
    self.stealing(bot, options).await;
  }

  pub fn stop(&self, nickname: &String) {
    kill_task(nickname, "stealer");
  }
}
