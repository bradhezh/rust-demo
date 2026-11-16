use std::io;

mod lifetime;
mod ownership;
mod smart_pointer;

pub fn run() {
  println!("1. Ownership and Borrowing");
  println!("2. Borrow Checking and Lifetimes");
  println!("3. Smart Pointers");

  let mut i = String::new();
  io::stdin().read_line(&mut i).expect("Failed to read line");
  let Ok(i) = i.trim().parse() as Result<u32, _> else {
    return;
  };
  match i {
    1 => ownership::run(),
    2 => lifetime::run(),
    3 => smart_pointer::run(),
    _ => return,
  }
}
