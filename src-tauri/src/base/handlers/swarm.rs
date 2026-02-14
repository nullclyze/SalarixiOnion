use azalea::swarm::*;

use crate::base::*;

pub async fn swarm_handler(swarm: Swarm, event: SwarmEvent, _state: NoSwarmState) {
  match event {
    SwarmEvent::Init => {
      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();
        fm.swarm = Some(swarm);
      }
    }
    _ => {}
  }
}
