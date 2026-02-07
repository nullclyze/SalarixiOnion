use rand::Rng;
use rand::prelude::SliceRandom;
use rand::thread_rng;


pub enum Classes {
  Numeric,
  Letter,
  Multi,
  Special
}

pub fn randchar(s: &str) -> Option<char> {
  let v: Vec<char> = s.chars().collect();
  if v.is_empty() { return None; }
  let idx = thread_rng().gen_range(0..v.len());
  Some(v[idx])
}

pub fn randstr(class: Classes, length: i32) -> String {
  let chars = match class {
    Classes::Numeric => "0123456789",
    Classes::Letter => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
    Classes::Multi => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    Classes::Special => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+="
  };

  let mut result = String::new();

  for _ in 0..length {
    result.push(randchar(chars).unwrap());
  }

  result
}

pub fn randint(min: i32, max: i32) -> i32 {
  thread_rng().gen_range(min..=max)
}

pub fn randuint(min: u64, max: u64) -> u64 {
  thread_rng().gen_range(min..=max)
}

pub fn randfloat(min: f64, max: f64) -> f64 {
  thread_rng().gen_range(min..=max)
}

pub fn randelem<T>(vec: &[T]) -> Option<&T> {
  vec.choose(&mut thread_rng())
}

pub fn randchance(percent: f64) -> bool {
  thread_rng().gen_bool(percent)
}