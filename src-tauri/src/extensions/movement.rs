use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::goals::XZGoal;
use azalea::pathfinder::moves::basic::basic_move;
use azalea::pathfinder::PathfinderOpts;
use azalea::{prelude::*, SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
use std::time::Duration;

use crate::core::*;
use crate::extensions::BotDefaultExt;

pub trait BotMovementExt {
  fn start_walking(&self, direction: WalkDirection);
  fn start_sprinting(&self, direction: SprintDirection);
  fn stop_move(&self);
  fn freeze_move(&self);
  fn unfreeze_move(&self);
}

impl BotMovementExt for Client {
  fn start_walking(&self, direction: WalkDirection) {
    let username = self.name();

    if get_state(&username, "can_walking")
      && !get_state(&username, "is_eating")
      && !get_state(&username, "is_drinking")
      && !get_state(&username, "is_interacting")
    {
      self.ecs.lock().write_message(StartWalkEvent {
        entity: self.entity,
        direction: direction,
      });

      set_mutual_states(&username, "walking", true);
    }
  }

  fn start_sprinting(&self, direction: SprintDirection) {
    let username = self.name();

    if get_state(&username, "can_sprinting")
      && !get_state(&username, "is_eating")
      && !get_state(&username, "is_drinking")
      && !get_state(&username, "is_interacting")
    {
      self.ecs.lock().write_message(StartSprintEvent {
        entity: self.entity,
        direction: direction,
      });

      set_mutual_states(&username, "sprinting", true);
    }
  }

  fn stop_move(&self) {
    self.ecs.lock().write_message(StartWalkEvent {
      entity: self.entity,
      direction: WalkDirection::None,
    });

    let username = self.name();

    set_mutual_states(&username, "walking", false);
    set_mutual_states(&username, "sprinting", false);
  }

  fn freeze_move(&self) {
    self.ecs.lock().write_message(StartWalkEvent {
      entity: self.entity,
      direction: WalkDirection::None,
    });

    let username = self.name();

    set_state(&username, "is_walking", false);
    set_state(&username, "is_walking", false);
    set_state(&username, "can_walking", false);
    set_state(&username, "can_sprinting", false);
  }

  fn unfreeze_move(&self) {
    let username = self.name();

    set_state(&username, "can_walking", true);
    set_state(&username, "can_sprinting", true);
  }
}

/// Функция безопасного задания координат по X и Z для бота
pub fn go_to(username: String, x: i32, z: i32) {
  if get_state(&username, "can_walking") && get_state(&username, "can_sprinting") {
    tokio::spawn(async move {
      BOT_REGISTRY
        .async_get_bot(&username, async |bot| {
          set_mutual_states(&username, "looking", true);
          set_mutual_states(&username, "sprinting", true);
          set_mutual_states(&username, "walking", true);

          let goal = XZGoal { x: x, z: z };
          let opts = PathfinderOpts::new()
            .min_timeout(PathfinderTimeout::Time(Duration::from_millis(500)))
            .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))
            .allow_mining(false)
            .successors_fn(basic_move);

          bot.goto_with_opts(goal, opts).await;

          set_mutual_states(&username, "looking", false);
          set_mutual_states(&username, "sprinting", false);
          set_mutual_states(&username, "walking", false);
        })
        .await;
    });
  }
}
