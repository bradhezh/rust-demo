use std::io;

mod cargo;
mod test;

pub fn run() {
  println!("1. Cargo");
  println!("2. Testing");

  let mut i = String::new();
  io::stdin().read_line(&mut i).expect("Failed to read line");
  let Ok(i) = i.trim().parse() as Result<u32, _> else {
    return;
  };
  match i {
    1 => cargo::run(),
    2 => test::run(),
    _ => return,
  }
}
