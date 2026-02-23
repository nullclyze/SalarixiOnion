use super::random::*;

pub fn mutate_text(text: String) -> String {
  let mut result = String::with_capacity(text.len());
  let mut chars = text.chars().peekable();

  while let Some(c) = chars.next() {
    if c == '#' {
      if let Some(&next) = chars.peek() {
        let r = match next {
          'n' => randstr(Classes::Numeric, 3),
          'l' => randstr(Classes::Letter, 3),
          'm' => randstr(Classes::Multi, 3),
          's' => randstr(Classes::Special, 3),
          _ => {
            result.push(c);
            continue;
          }
        };

        chars.next();
        result.push_str(&r);

        continue;
      }
    }

    result.push(c);
  }

  result
}
