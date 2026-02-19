use azalea::player::GameProfileComponent;
use chrono::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

use crate::base::{BOT_REGISTRY, PROFILES};
use crate::common::get_entity_position;
use crate::methods::SafeClientMethods;

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

  pub async fn find_target(&self, target: String) -> Option<RadarInfo> {
    for username in PROFILES.get_all().keys() {
      let info = BOT_REGISTRY
        .get_bot(username, async |bot| {
          let Some(tab_list) = bot.get_players() else {
            return None;
          };

          for uuid in tab_list.keys() {
            let Some(entity) = bot.entity_by_uuid(*uuid) else {
              continue;
            };

            let Some(profile) = bot.get_entity_component::<GameProfileComponent>(entity) else {
              continue;
            };

            if profile.0.name == target {
              let player_pos = get_entity_position(bot, entity);
              let client_pos = bot.feet_pos();

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

          None
        })
        .await;

      if let Some(radar_info) = info {
        return radar_info;
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
