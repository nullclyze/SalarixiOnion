use azalea::entity::Position;
use azalea::local_player::TabList;
use azalea::player::GameProfileComponent;
use chrono::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

use crate::base::get_flow_manager;

pub static RADAR_MANAGER: Lazy<Arc<RadarManager>> = Lazy::new(|| Arc::new(RadarManager::new()));

// Структура информации радара
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarInfo {
  pub status: String,
  pub uuid: String,
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub observer: RadarObserver,
}

// Структура информации о наблюдателе
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarObserver {
  pub x: f64,
  pub z: f64,
}

// Структура RadarManager
pub struct RadarManager;

impl RadarManager {
  pub fn new() -> Self {
    Self
  }

  pub fn find_target(&self, target: String) -> Option<RadarInfo> {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.read();

      if fm.active && fm.bots.len() > 0 {
        for (_, bot) in &fm.bots {
          if let Some(tab_list) = bot.get_component::<TabList>() {
            for uuid in tab_list.keys() {
              if let Some(entity) = bot.entity_by_uuid(*uuid) {
                if let Some(profile) = bot.get_entity_component::<GameProfileComponent>(entity) {
                  if profile.0.name == target {
                    if let Some(player_pos) = bot.get_entity_component::<Position>(entity) {
                      let client_pos = bot.position();

                      return Some(RadarInfo {
                        status: "Найден".to_string(),
                        uuid: uuid.to_string(),
                        x: player_pos.x,
                        y: player_pos.y,
                        z: player_pos.z,
                        observer: RadarObserver {
                          x: client_pos.x,
                          z: client_pos.z,
                        },
                      });
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    None
  }

  pub fn save_data(
    &self,
    target: String,
    mut path: String,
    filename: String,
    x: f64,
    y: f64,
    z: f64,
  ) {
    let date = Local::now().format("%H:%M:%S").to_string();
    let content = format!("[ {} ] {} ~ X: {}, Y: {}, Z: {}", date, target, x, y, z);

    path.push_str(&filename.replace("#t", target.as_str()));
    path.push_str(".txt");

    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&path)
      .unwrap();

    writeln!(&mut file, "{}", content).unwrap();
  }
}
