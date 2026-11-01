use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
  let secret = rand::rng().random_range(1..=100);
  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      // Note that `=>` is a separator between a pattern and an arm. A closure is like `|x| x + 1`.
      Err(_) => continue,
      Ok(n) => n,
    };
    println!("Your guess: {guess}");

    match guess.cmp(&secret) {
      Ordering::Equal => {
        println!("You win!");
        break;
      }
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
    }
  }
}
