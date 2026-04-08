use azalea::swarm::*;

use crate::service::core::registry::BOT_REGISTRY;

pub async fn swarm_handler(swarm: Swarm, event: SwarmEvent, _state: NoSwarmState) {
  match event {
    SwarmEvent::Init => {
      BOT_REGISTRY.set_swarm(swarm).await;
    }
    _ => {}
  }
}
