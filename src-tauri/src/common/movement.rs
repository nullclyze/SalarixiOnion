use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::goals::XZGoal;
use azalea::pathfinder::moves::basic::basic_move;
use azalea::pathfinder::PathfinderOpts;
use azalea::prelude::*;
use azalea::SprintDirection;
use azalea::WalkDirection;
use std::time::Duration;

use crate::base::*;

/// Функция остановки движения бота
pub fn stop_bot_move(bot: &Client) {
  let nickname = bot.username();

  bot.stop_pathfinding();
  bot.walk(WalkDirection::None);

  STATES.set_mutual_states(&nickname, "walking", false);
  STATES.set_mutual_states(&nickname, "sprinting", false);
}

/// Функция безопасного задания направления хотьбы для бота
pub fn go(bot: &Client, direction: WalkDirection) {
  let nickname = bot.username();

  if STATES.get_state(&nickname, "can_walking") {
    STATES.set_mutual_states(&nickname, "walking", true);
    bot.walk(direction);
  }
}

/// Функция безопасного задания направления бега для бота
pub fn run(bot: &Client, direction: SprintDirection) {
  let nickname = bot.username();

  if STATES.get_state(&nickname, "can_sprinting") {
    STATES.set_mutual_states(&nickname, "sprinting", true);
    bot.sprint(direction);
  }
}

/// Функция безопасного задания координат по X и Z для бота
pub fn go_to(bot: Client, x: i32, z: i32) {
  let nickname = bot.username();

  if STATES.get_state(&nickname, "can_walking") && STATES.get_state(&nickname, "can_sprinting") {
    tokio::spawn(async move {
      STATES.set_mutual_states(&nickname, "sprinting", true);
      STATES.set_mutual_states(&nickname, "walking", true);

      let goal = XZGoal { x: x, z: z };
      let opts = PathfinderOpts::new()
        .min_timeout(PathfinderTimeout::Time(Duration::from_millis(500)))
        .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))
        .allow_mining(false)
        .successors_fn(basic_move);

      bot.goto_with_opts(goal, opts).await;

      STATES.set_mutual_states(&nickname, "sprinting", false);
      STATES.set_mutual_states(&nickname, "walking", false);
    });
  }
}
