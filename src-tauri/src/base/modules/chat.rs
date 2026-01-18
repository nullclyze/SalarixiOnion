use azalea::prelude::*;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatModule;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMode {
  Message,
  Spamming
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOptions {
  pub mode: ChatMode,
  pub message: String,
  pub use_text_mutation: bool,
  pub use_sync: bool,
  pub use_magic_text: bool,
  pub use_bypass: bool,
  pub use_anti_repetition: bool,
  pub min_delay: Option<u64>,
  pub max_delay: Option<u64>,
  pub state: bool
}

impl ChatModule {
  fn create_magic_text(text: &str) -> String {
    let mut result = String::new();

    let words = text.split_whitespace();

    for word in words {
      if !result.is_empty() {
        result.push(' ');
      }

      if word.contains("http://") || word.contains("https://") || word.contains("@") {
        result.push_str(word);
        continue;
      }

      let random_chance = randfloat(0.0, 1.0);

      if random_chance >= 0.9 {
        result.push_str(&word.to_lowercase());
      } else if random_chance < 0.9 && random_chance > 0.75 {
        result.push_str(&word.to_uppercase());
      } else {
        for char in word.chars() {
          let char_chance = randfloat(0.0, 1.0);

          if char_chance >= 0.70 {
            let mut transformed = char.to_lowercase().to_string();
                        
            transformed = transformed
              .replace("o", if randchance(0.5) { "0" } else { "@" })
              .replace("о", if randchance(0.5) { "0" } else { "@" })
              .replace("a", "4")
              .replace("а", "4")
              .replace("z", "3")
              .replace("з", "3")
              .replace("e", "3")
              .replace("е", "3")
              .replace("i", if randchance(0.5) { "1" } else { "!" })
              .replace("l", if randchance(0.5) { "1" } else { "!" })
              .replace("л", if randchance(0.5) { "1" } else { "!" })
              .replace("и", if randchance(0.5) { "1" } else { "!" })
              .replace("п", "5")
              .replace("p", "5")
              .replace("v", if randchance(0.5) { "8" } else { "&" })
              .replace("в", if randchance(0.5) { "8" } else { "&" })
              .replace("б", "6")
              .replace("b", "6")
              .replace("с", "$")
              .replace("s", "$");
                        
            result.push_str(&transformed);
          } else if char_chance < 0.70 && char_chance >= 0.5 {
            result.push_str(&char.to_uppercase().to_string());
          } else {
            result.push(char);
          }
        }
      }
    }

    result
  } 

  fn create_bypass_text(text: &str) -> String {
    let stray = ["numeric", "letter", "multi", "special"];
    let separators = ["|", ":", "/", "~"];

    let random_stray = randelem(&stray).unwrap();
    let random_separator = randelem(&separators).unwrap();

    let stray_text = match random_stray {
      &"numeric" => randstr(Classes::Numeric, randint(3, 7)),
      &"letter" => randstr(Classes::Letter, randint(3, 7)),
      &"multi" => randstr(Classes::Multi, randint(3, 7)),
      &"special" => randstr(Classes::Special, randint(3, 7)),
      _ => String::new()
    };

    if randchance(0.3) {
      format!("{} {} {}", text, random_separator, stray_text)
    } else {
      format!("{} {} {}", stray_text, random_separator, text)
    }
  }

  pub async fn message(bot: &Client, options: ChatOptions) -> anyhow::Result<()> {
    let mut text = options.message.clone();

    if options.use_text_mutation {
      text = Mutator::mutate_text(text);
    }

    if !options.use_sync {
      sleep(Duration::from_millis(randuint(200, 4000))).await;
    }

    if options.use_magic_text {
      text = Self::create_magic_text(&text);
    }

    bot.chat(&text);

    Ok(())
  }

  pub async fn spamming(bot: &Client, options: ChatOptions) {
    let mut latest_text = String::new();

    loop {
      let mut text = options.message.clone();

      if options.use_text_mutation {
        text = Mutator::mutate_text(text);
      }

      if options.use_sync {
        sleep(Duration::from_millis(options.min_delay.unwrap_or(2000) + options.min_delay.unwrap_or(4000) / 2)).await;
      } else {
        sleep(Duration::from_millis(randuint(options.min_delay.unwrap_or(2000), options.max_delay.unwrap_or(4000)))).await;
      }

      let mut final_text = text.clone();
                  
      if options.use_magic_text {
        final_text = Self::create_magic_text(&final_text);
      }
                  
      if options.use_bypass {
        final_text = Self::create_bypass_text(&final_text);
      }

      if options.use_anti_repetition {
        if latest_text != final_text.clone() {
          bot.chat(&final_text);
          latest_text = final_text.clone();
        }
      } else {
        bot.chat(&final_text);
      }
    }
  }

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("spamming");
  }
}