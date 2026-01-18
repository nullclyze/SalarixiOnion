use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos;
use azalea::container::ContainerHandle;
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealerOptions {
  pub target: String,
  pub radius: Option<i32>,
  pub delay: Option<usize>,
  pub state: bool
}

impl StealerModule {
  fn check_block_id(block_id: u16, target: &String) -> bool {
    match target.as_str() {
      "chest" => {
        let id_list = vec![3793, 3787, 3805, 3799];

        for id in id_list {
          if block_id == id {
            return true;
          }
        }
      },
      "barrel" => {
        let id_list = vec![20547, 20543, 20541, 20549, 20545];

        for id in id_list {
          if block_id == id {
            return true;
          }
        }
      },
      "shulker" => {
        let id_list = vec![
          14666, 14672, 14678, 14720, 
          14756, 14714, 14762, 14744,
          14702, 14696, 14708, 14750,
          14684, 14726, 14738, 14732
        ];

        for id in id_list {
          if block_id == id {
            return true;
          }
        }
      },
      _ => {}
    }

    false
  }

  fn find_nearest_targets(bot: &Client, center: Vec3, target: &String, radius: i32) -> Vec<azalea::BlockPos> {
    let mut positions = Vec::new();  

    let world_clone = bot.world();
    let world = world_clone.read();
        
    for x in -radius..=radius {  
      for y in -radius..=radius {  
        for z in -radius..=radius {  
          let block_pos = BlockPos::new(  
            (center.x as i32 + x) as i32,  
            (center.y as i32 + y) as i32,  
            (center.z as i32 + z) as i32,  
          );  
                    
          if let Some(block_state) = world.get_block_state(block_pos) {  
            let block_id = block_state.id();  

            if Self::check_block_id(block_id, target) {  
              positions.push(block_pos);  
            }  
          } 
        } 
      }  
    }
      
    positions  
  }

  async fn extract_all_items(container: &ContainerHandle) {  
    let menu = container.menu().unwrap();  
       
    for slot in 0..=26 {  
      if let Some(item) = menu.slot(slot) {  
        if !item.is_empty() {  
          container.shift_click(slot); 
        }  
      }  
    }  
  }

  pub async fn enable(bot: &Client, options: StealerOptions) {
    loop {
      let position = bot.position();  
      let direction = bot.direction();

      let target_positions = Self::find_nearest_targets(bot, position.clone(), &options.target, if let Some(radius) = options.radius { radius } else { 4 });
        
      for pos in target_positions {  
        bot.look_at(pos.center());  

        bot.wait_ticks(randuint(1, 2) as usize).await;
            
        if let Some(container) = bot.open_container_at(pos).await {  
          Self::extract_all_items(&container).await;  
          bot.wait_ticks(randuint(4, 6) as usize).await;
        }  
      } 

      bot.set_direction(direction.0, direction.1);

      bot.wait_ticks(options.delay.unwrap_or_else(|| { 20 })).await;
    }
  } 

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("stealer");
  }
}