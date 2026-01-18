use regex::Regex;


pub fn extract_link_from_message(message: String) -> Option<String> {
  let re = Regex::new(r"https?://[^\s]+").unwrap();

  for link_to_captcha in re.find_iter(&message) {
    if !link_to_captcha.is_empty() {
      return Some(link_to_captcha.as_str().to_string());
    }
  }

  None  
}