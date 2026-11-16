use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

pub fn run() {
  guessing_game();
}

fn guessing_game() {
  let secret = rand::rng().random_range(1..=100);
  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      // Note that `=>` is a separator between the pattern and its code of an arm. A closure is like
      // `|x| x + 1`.
      Ok(n) => n,
      Err(_) => continue,
    };
    println!("Your guess: {guess}");

    match guess.cmp(&secret) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
