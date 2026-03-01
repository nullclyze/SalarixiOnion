use base64::encode;
use image::{ImageBuffer, ImageFormat, Rgb};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::{Arc, RwLock};
use thirtyfour::{
  common::capabilities::firefox::FirefoxPreferences, error::WebDriverError, CapabilitiesHelper,
  ChromiumLikeCapabilities, DesiredCapabilities, WebDriver,
};
use tokio::sync::broadcast;

use crate::{core::current_options, emit::send_log, generators::randint};

pub static WEB_CAPTCHA_BYPASS: Lazy<Arc<WebCaptchaBypass>> =
  Lazy::new(|| Arc::new(WebCaptchaBypass::new()));
pub static MAP_CAPTCHA_BYPASS: Lazy<Arc<MapCaptchaBypass>> =
  Lazy::new(|| Arc::new(MapCaptchaBypass::new()));

/// Обход web-капчи
pub struct WebCaptchaBypass {
  pub webdriver_events: broadcast::Sender<WebDriverEvent>,
  pub active_tabs_count: RwLock<i32>,
}

#[derive(Clone)]
pub enum WebDriverEvent {
  OpenUrl {
    url: String,
    proxy: Option<String>,
    username: Option<String>,
    password: Option<String>,
  },
  CreateWebDriver {
    proxy: Option<String>,
    username: Option<String>,
    password: Option<String>,
  },
  StopProcessing,
}

impl WebCaptchaBypass {
  pub fn new() -> Self {
    let (tx, _) = broadcast::channel(1000);

    Self {
      webdriver_events: tx,
      active_tabs_count: RwLock::new(0),
    }
  }

  pub fn catch_url_from_message(
    &self,
    message: String,
    regex: &str,
    required_url_part: Option<String>,
  ) -> Option<String> {
    let re = Regex::new(regex).unwrap();

    for link_to_captcha in re.find_iter(&message) {
      if !link_to_captcha.is_empty() {
        if let Some(required) = required_url_part.clone() {
          if link_to_captcha.as_str().contains(required.as_str()) {
            return Some(link_to_captcha.as_str().to_string());
          }
        } else {
          return Some(link_to_captcha.as_str().to_string());
        }
      }
    }

    None
  }

  fn random_user_agent(&self) -> String {
    let user_agents = vec![
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36",
      "Mozilla/5.0 (Windows; U; Windows NT 5.0) AppleWebKit/537.2.1 (KHTML, like Gecko) Chrome/33.0.854.0 Safari/537.2.1",
      "Mozilla/5.0 (Windows NT 5.2; rv:8.6) Gecko/20100101 Firefox/8.6.8",
      "Mozilla/5.0 (Windows; U; Windows NT 5.0) AppleWebKit/531.2.1 (KHTML, like Gecko) Chrome/30.0.851.0 Safari/531.2.1",
      "Mozilla/5.0 (Windows; U; Windows NT 6.2) AppleWebKit/532.1.1 (KHTML, like Gecko) Chrome/39.0.889.0 Safari/532.1.1",
      "Mozilla/5.0 (Windows NT 6.0; rv:7.3) Gecko/20100101 Firefox/7.3.6",
      "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_9 rv:2.0; MS) AppleWebKit/537.1.1 (KHTML, like Gecko) Version/7.0.5 Safari/537.1.1",
      "Mozilla/5.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/7.0; .NET CLR 1.1.37036.4)",
      "Mozilla/5.0 (Windows; U; Windows NT 6.1) AppleWebKit/531.2.2 (KHTML, like Gecko) Chrome/17.0.888.0 Safari/531.2.2",
      "Mozilla/5.0 (Windows; U; Windows NT 5.3) AppleWebKit/535.1.0 (KHTML, like Gecko) Chrome/35.0.864.0 Safari/535.1.0",
      "Opera/12.49 (Windows NT 5.3; U; SK) Presto/2.9.168 Version/10.00",
      "Mozilla/5.0 (Windows; U; Windows NT 6.2) AppleWebKit/538.0.2 (KHTML, like Gecko) Chrome/21.0.890.0 Safari/538.0.2",
      "Mozilla/5.0 (Windows; U; Windows NT 5.0) AppleWebKit/537.0.0 (KHTML, like Gecko) Chrome/37.0.899.0 Safari/537.0.0",
      "Mozilla/5.0 (Windows; U; Windows NT 6.0) AppleWebKit/536.1.1 (KHTML, like Gecko) Chrome/27.0.815.0 Safari/536.1.1",
      "Mozilla/5.0 (Windows; U; Windows NT 5.3) AppleWebKit/533.0.2 (KHTML, like Gecko) Chrome/35.0.884.0 Safari/533.0.2",
      "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_7 rv:3.0; NN) AppleWebKit/534.2.2 (KHTML, like Gecko) Version/4.0.6 Safari/534.2.2",
      "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_8_9 rv:5.0; ID) AppleWebKit/533.2.0 (KHTML, like Gecko) Version/4.1.5 Safari/533.2.0",
      "Mozilla/5.0 (Windows NT 6.0; Win64; rv:14.0) Gecko/20100101 Firefox/14.0.0"
    ];

    let idx = (randint(1, user_agents.len() as i32) - 1) as usize;
    let random_user_agent = user_agents[idx];

    random_user_agent.to_string()
  }

  pub async fn create_webdriver(
    &self,
    browser: String,
    server_url: Option<String>,
    proxy: Option<thirtyfour::Proxy>,
  ) -> Result<WebDriver, WebDriverError> {
    let url = server_url.unwrap_or("http://localhost:9515".to_string());

    match browser.as_str() {
      "firefox" => {
        let mut caps = DesiredCapabilities::firefox();

        if let Some(p) = proxy {
          caps.set_proxy(p)?;
        }

        let mut prefs = FirefoxPreferences::new();
        prefs.set_user_agent(self.random_user_agent())?;
        caps.set_preferences(prefs)?;

        // let _ = caps.add_arg("--headless");
        return WebDriver::new(url, caps).await;
      }
      "edge" => {
        let mut caps = DesiredCapabilities::edge();

        if let Some(p) = proxy {
          caps.set_proxy(p)?;
        }

        caps.add_arg(&format!("--user-agent={}", self.random_user_agent()))?;

        // let _ = caps.add_arg("--headless");
        return WebDriver::new(url, caps).await;
      }
      "chrome" => {
        let mut caps = DesiredCapabilities::chrome();

        if let Some(p) = proxy {
          caps.set_proxy(p)?;
        }

        caps.add_arg(&format!("--user-agent={}", self.random_user_agent()))?;

        // let _ = caps.add_arg("--headless");
        return WebDriver::new(url, caps).await;
      }
      "opera" => {
        let mut caps = DesiredCapabilities::opera();

        if let Some(p) = proxy {
          caps.set_proxy(p)?;
        }

        caps.add_arg(&format!("--user-agent={}", self.random_user_agent()))?;

        // let _ = caps.add_arg("--headless");
        return WebDriver::new(url, caps).await;
      }
      "chromium" => {
        let mut caps = DesiredCapabilities::chromium();

        if let Some(p) = proxy {
          caps.set_proxy(p)?;
        }

        caps.add_arg(&format!("--user-agent={}", self.random_user_agent()))?;

        // let _ = caps.add_arg("--headless");
        return WebDriver::new(url, caps).await;
      }
      _ => return Err(WebDriverError::FatalError("Incorrect browser".to_string())),
    }
  }

  pub fn send_webdriver_event(&self, event: WebDriverEvent) {
    let _ = self.webdriver_events.send(event);
  }

  pub async fn webdriver_event_loop(&'static self) {
    let mut webdriver: Option<WebDriver> = None;

    let mut rx = self.webdriver_events.subscribe();
    let mut main_window = None;

    while let Ok(event) = rx.recv().await {
      match event {
        WebDriverEvent::OpenUrl {
          url,
          proxy,
          username,
          password,
        } => {
          if *self.active_tabs_count.read().unwrap() >= 3 {
            self.send_webdriver_event(WebDriverEvent::CreateWebDriver {
              proxy: proxy,
              username: username,
              password: password,
            });

            continue;
          }

          if let Some(driver) = webdriver.as_ref() {
            let _ = driver.delete_all_cookies().await;

            if main_window.is_none() {
              if let Ok(handle) = driver.window().await {
                main_window = Some(handle);
              }
            }
            if let Ok(new_handle) = driver.new_tab().await {
              if driver.switch_to_window(new_handle).await.is_ok() {
                let _ = driver.goto(url).await;

                *self.active_tabs_count.write().unwrap() += 1;
              }
            }
          }
        }
        WebDriverEvent::CreateWebDriver {
          proxy,
          username,
          password,
        } => {
          if let Some(driver) = webdriver.clone() {
            let _ = driver.quit().await;
            *self.active_tabs_count.write().unwrap() = 0;
          }

          if let Some(options) = current_options() {
            let mut manual_proxy = None;

            if let Some(p) = proxy {
              manual_proxy = Some(thirtyfour::Proxy::Manual {
                ftp_proxy: None,
                http_proxy: None,
                ssl_proxy: None,
                socks_proxy: Some(p),
                socks_version: Some(5),
                socks_username: username,
                socks_password: password,
                no_proxy: None,
              });
            }

            match self
              .create_webdriver(
                options.captcha_bypass.browser,
                options.captcha_bypass.webdriver_server_url,
                manual_proxy,
              )
              .await
            {
              Ok(driver) => {
                webdriver = Some(driver);
                main_window = None;
              }
              Err(err) => {
                send_log(format!("Ошибка создания WebDriver: {}", err), "error");
              }
            }
          }
        }
        WebDriverEvent::StopProcessing => {
          if let Some(driver) = webdriver.clone() {
            let _ = driver.quit().await;
          }

          return;
        }
      }
    }
  }
}

/// Обход map-капчи
pub struct MapCaptchaBypass;

impl MapCaptchaBypass {
  pub fn new() -> Self {
    Self
  }

  fn convert_id_to_rgb_color(&self, id: u8) -> (u8, u8, u8) {
    match id {
      0 => (255, 255, 255),
      1 => (255, 255, 255),
      2 => (255, 255, 255),
      3 => (255, 255, 255),
      4 => (89, 125, 39),
      5 => (109, 153, 48),
      6 => (127, 178, 56),
      7 => (67, 94, 29),
      8 => (174, 164, 115),
      9 => (213, 201, 140),
      10 => (247, 233, 163),
      11 => (130, 123, 86),
      12 => (140, 140, 140),
      13 => (171, 171, 171),
      14 => (199, 199, 199),
      15 => (105, 105, 105),
      16 => (180, 0, 0),
      17 => (220, 0, 0),
      18 => (255, 0, 0),
      19 => (135, 0, 0),
      20 => (112, 112, 180),
      21 => (138, 138, 220),
      22 => (160, 160, 255),
      23 => (84, 84, 135),
      24 => (117, 117, 117),
      25 => (144, 144, 144),
      26 => (167, 167, 167),
      27 => (88, 88, 88),
      28 => (0, 87, 0),
      29 => (0, 106, 0),
      30 => (0, 124, 0),
      31 => (0, 65, 0),
      32 => (180, 180, 180),
      33 => (220, 220, 220),
      34 => (255, 255, 255),
      35 => (135, 135, 135),
      36 => (115, 118, 129),
      37 => (141, 144, 158),
      38 => (164, 168, 184),
      39 => (86, 88, 97),
      40 => (106, 76, 54),
      41 => (130, 94, 66),
      42 => (151, 109, 77),
      43 => (79, 57, 40),
      44 => (79, 79, 79),
      45 => (96, 96, 96),
      46 => (112, 112, 112),
      47 => (59, 59, 59),
      48 => (45, 45, 180),
      49 => (55, 55, 220),
      50 => (64, 64, 255),
      51 => (33, 33, 135),
      52 => (100, 84, 50),
      53 => (123, 102, 62),
      54 => (143, 119, 72),
      55 => (75, 63, 38),
      56 => (180, 177, 172),
      57 => (220, 217, 211),
      58 => (255, 252, 245),
      59 => (135, 133, 129),
      60 => (152, 89, 36),
      61 => (186, 109, 44),
      62 => (216, 127, 51),
      63 => (114, 67, 27),
      64 => (125, 53, 152),
      65 => (153, 65, 186),
      66 => (178, 76, 216),
      67 => (94, 40, 114),
      68 => (72, 108, 152),
      69 => (88, 132, 186),
      70 => (102, 153, 216),
      71 => (54, 81, 114),
      72 => (161, 161, 36),
      73 => (197, 197, 44),
      74 => (229, 229, 51),
      75 => (121, 121, 27),
      76 => (89, 144, 17),
      77 => (109, 176, 21),
      78 => (127, 204, 25),
      79 => (67, 108, 13),
      80 => (170, 89, 116),
      81 => (208, 109, 142),
      82 => (242, 127, 165),
      83 => (128, 67, 87),
      84 => (53, 53, 53),
      85 => (65, 65, 65),
      86 => (76, 76, 76),
      87 => (40, 40, 40),
      88 => (108, 108, 108),
      89 => (132, 132, 132),
      90 => (153, 153, 153),
      91 => (81, 81, 81),
      92 => (53, 89, 108),
      93 => (65, 109, 132),
      94 => (76, 127, 153),
      95 => (40, 67, 81),
      96 => (89, 44, 125),
      97 => (109, 54, 153),
      98 => (127, 63, 178),
      99 => (67, 33, 94),
      100 => (36, 53, 125),
      101 => (44, 65, 153),
      102 => (51, 76, 178),
      103 => (27, 40, 94),
      104 => (72, 53, 36),
      105 => (88, 65, 44),
      106 => (102, 76, 51),
      107 => (54, 40, 27),
      108 => (72, 89, 36),
      109 => (88, 109, 44),
      110 => (102, 127, 51),
      111 => (54, 67, 27),
      112 => (108, 36, 36),
      113 => (132, 44, 44),
      114 => (153, 51, 51),
      115 => (81, 27, 27),
      116 => (17, 17, 17),
      117 => (21, 21, 21),
      118 => (25, 25, 25),
      119 => (13, 13, 13),
      120 => (176, 168, 54),
      121 => (215, 205, 66),
      122 => (250, 238, 77),
      123 => (132, 126, 40),
      124 => (64, 154, 150),
      125 => (79, 188, 183),
      126 => (92, 219, 213),
      127 => (48, 115, 112),
      128 => (52, 90, 180),
      129 => (63, 110, 220),
      130 => (74, 128, 255),
      131 => (39, 67, 135),
      132 => (0, 153, 40),
      133 => (0, 187, 50),
      134 => (0, 217, 58),
      135 => (0, 114, 30),
      136 => (91, 60, 34),
      137 => (111, 74, 42),
      138 => (129, 86, 49),
      139 => (68, 45, 25),
      140 => (79, 1, 0),
      141 => (96, 1, 0),
      142 => (112, 2, 0),
      143 => (59, 1, 0),
      144 => (147, 124, 113),
      145 => (180, 152, 138),
      146 => (209, 177, 161),
      147 => (110, 93, 85),
      148 => (112, 57, 25),
      149 => (137, 70, 31),
      150 => (159, 82, 36),
      151 => (84, 43, 19),
      152 => (105, 61, 76),
      153 => (128, 75, 93),
      154 => (149, 87, 108),
      155 => (78, 46, 57),
      156 => (79, 76, 97),
      157 => (96, 93, 119),
      158 => (112, 108, 138),
      159 => (59, 57, 73),
      160 => (131, 93, 25),
      161 => (160, 114, 31),
      162 => (186, 133, 36),
      163 => (98, 70, 19),
      164 => (72, 82, 37),
      165 => (88, 100, 45),
      166 => (103, 117, 53),
      167 => (54, 61, 28),
      168 => (112, 54, 55),
      169 => (138, 66, 67),
      170 => (160, 77, 78),
      171 => (84, 40, 41),
      172 => (40, 28, 24),
      173 => (49, 35, 30),
      174 => (57, 41, 35),
      175 => (30, 21, 18),
      176 => (95, 75, 69),
      177 => (116, 92, 84),
      178 => (135, 107, 98),
      179 => (71, 56, 51),
      180 => (61, 64, 64),
      181 => (75, 79, 79),
      182 => (87, 92, 92),
      183 => (46, 48, 48),
      184 => (86, 51, 62),
      185 => (105, 62, 75),
      186 => (122, 73, 88),
      187 => (64, 38, 46),
      188 => (53, 43, 64),
      189 => (65, 53, 79),
      190 => (76, 62, 92),
      191 => (40, 32, 48),
      192 => (53, 35, 24),
      193 => (65, 43, 30),
      194 => (76, 50, 35),
      195 => (40, 26, 18),
      196 => (53, 57, 29),
      197 => (65, 70, 36),
      198 => (76, 82, 42),
      199 => (40, 43, 22),
      200 => (100, 42, 32),
      201 => (122, 51, 39),
      202 => (142, 60, 46),
      203 => (75, 31, 24),
      204 => (26, 15, 11),
      205 => (31, 18, 13),
      206 => (37, 22, 16),
      207 => (19, 11, 8),
      _ => (255, 255, 255),
    }
  }

  pub fn create_png_image(&self, map: &Vec<u8>) -> String {
    let width = 128;
    let height = 128;

    let mut img = ImageBuffer::new(width, height);

    for (i, &id) in map.iter().enumerate() {
      let rgb = self.convert_id_to_rgb_color(id);
      let x = (i % 128) as u32;
      let y = (i / 128) as u32;

      img.put_pixel(x, y, Rgb([rgb.0, rgb.1, rgb.2]));
    }

    let mut bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);

    let _ = img.write_to(&mut cursor, ImageFormat::Png);

    let base64_code = encode(&bytes);

    base64_code
  }
}
