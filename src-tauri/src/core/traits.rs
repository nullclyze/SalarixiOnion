use std::io;

pub trait SalarixiPlugin {
  fn new() -> Self;
  fn activate(&'static self, username: String) -> io::Result<()>;
}