use std::io;

mod closure;
mod collection;
mod error;
mod structs;

pub fn run() {
  println!("1. Structs and Enums");
  println!("2. Collections");
  println!("3. Error Handling");
  println!("4. Closures and Iterators");

  let mut i = String::new();
  io::stdin().read_line(&mut i).expect("Failed to read line");
  let Ok(i) = i.trim().parse() as Result<u32, _> else {
    return;
  };
  match i {
    1 => structs::run(),
    2 => collection::run(),
    3 => error::run(),
    4 => closure::run(),
    _ => return,
  }
}
