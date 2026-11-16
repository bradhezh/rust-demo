use std::io;

mod concurrency;
mod data_op;
mod generic;
mod memory;
mod unsafes;

pub fn run() {
  println!("1. Memory Management");
  println!("2. Data and Operations");
  println!("3. Concurrency and Async");
  println!("4. Generics and Macros");
  println!("5. Unsafe Rust");

  let mut i = String::new();
  io::stdin().read_line(&mut i).expect("Failed to read line");
  let Ok(i) = i.trim().parse() as Result<u32, _> else {
    return;
  };
  match i {
    1 => memory::run(),
    2 => data_op::run(),
    3 => concurrency::run(),
    4 => generic::run(),
    5 => unsafes::run(),
    _ => return,
  }
}
