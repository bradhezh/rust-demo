use std::io;

mod lang;
mod quickstart;
mod toolchain;

fn main() {
  println!("1. Quickstart");
  println!("2. The Language");
  println!("3. Toolchain");

  let mut i = String::new();
  io::stdin().read_line(&mut i).expect("Failed to read line");
  let Ok(i) = i.trim().parse() as Result<u32, _> else {
    return;
  };
  match i {
    1 => quickstart::run(),
    2 => lang::run(),
    3 => toolchain::run(),
    _ => return,
  }
}
