use azalea::ping;
use dns_lookup::lookup_host;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  username: String,
  uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInformation {
  pub ip_address: String,
  pub server_icon: Option<String>,
  pub protocol_version: i32,
  pub server_version: String,
  pub description: String,
  pub players_online: i32,
  pub players_max: i32,
  pub list_of_players: Vec<Player>,
}

pub async fn ping_server(address: String) -> ServerInformation {
  let mut info = ServerInformation {
    ip_address: "?".to_string(),
    server_icon: None,
    protocol_version: -1,
    server_version: "?".to_string(),
    description: "?".to_string(),
    players_online: 0,
    players_max: 0,
    list_of_players: Vec::new(),
  };

  let split_address: Vec<&str> = address.split(":").collect();

  if let Some(host) = split_address.get(0) {
    let ip_response = lookup_host(*host);

    match ip_response {
      Ok(resp) => {
        let vector: Vec<IpAddr> = resp.collect();
        info.ip_address = vector
          .get(0)
          .unwrap_or(&IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
          .to_string();
      }
      Err(_) => {}
    }
  }

  let ping_response = ping::ping_server(address).await;

  match ping_response {
    Ok(resp) => {
      info.server_icon = resp.favicon;
      info.protocol_version = resp.version.protocol;
      info.server_version = resp.version.name;
      info.description = resp.description.to_html();
      info.players_online = resp.players.online;
      info.players_max = resp.players.max;

      for player in resp.players.sample {
        info.list_of_players.push(Player {
          username: player.name,
          uuid: player.id,
        });
      }
    }
    Err(_) => {}
  }

  info
}
