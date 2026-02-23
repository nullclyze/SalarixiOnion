use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::goals::XZGoal;
use azalea::pathfinder::moves::basic::basic_move;
use azalea::pathfinder::PathfinderOpts;
use azalea::prelude::*;
use std::time::Duration;

use crate::core::*;

/// Функция безопасного задания координат по X и Z для бота
pub fn go_to(username: String, x: i32, z: i32) {
  if STATES.get_state(&username, "can_walking") && STATES.get_state(&username, "can_sprinting") {
    tokio::spawn(async move {
      BOT_REGISTRY
        .get_bot(&username, async |bot| {
          STATES.set_mutual_states(&username, "looking", true);
          STATES.set_mutual_states(&username, "sprinting", true);
          STATES.set_mutual_states(&username, "walking", true);

          let goal = XZGoal { x: x, z: z };
          let opts = PathfinderOpts::new()
            .min_timeout(PathfinderTimeout::Time(Duration::from_millis(500)))
            .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))
            .allow_mining(false)
            .successors_fn(basic_move);

          bot.goto_with_opts(goal, opts).await;

          STATES.set_mutual_states(&username, "looking", false);
          STATES.set_mutual_states(&username, "sprinting", false);
          STATES.set_mutual_states(&username, "walking", false);
        })
        .await;
    });
  }
}
