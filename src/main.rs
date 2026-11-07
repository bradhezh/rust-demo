use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
  //guessing_game();
  ownership();
  let s = String::from("hello");
  // Borrow: Without taking ownership
  refs(&s);
  //hello
  println!("{s}");
  reborrow();
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

fn ownership() {
  // Variables holding heap values
  let mut s1 = String::from("hello");
  let s2 = String::from("world");
  //hello
  println!("{s1}");
  // Move: s2 is invalidated as the original owner of "world"; "hello" losing its owner is dropped.
  s1 = s2;
  // s1 is the owner of "world".
  //world
  println!("{s1}");
  /* s2 is invalid. Error: borrow of moved value: s2
  println!("{s2}");
  */

  // Stack variables
  let a = (0, 'a');
  // Copy
  let mut b = a;
  b.1 = 'b';
  //(0, 'a'), (0, 'b')
  println!("{:?}, {:?}", a, b)
} // s1 drops "world" right before it goes out of scope.

static G_VAR: i32 = 5;

// The arg can also be &String due to deref coercion.
fn refs(s: &str) {
  // Point to a static variable (&i32)
  let r1 = &G_VAR;
  //5
  println!("{r1}");

  let mut s = String::from(s);
  // Point to a heap string value (&mut str)
  let r2 = &mut s[..1];
  r2.make_ascii_uppercase();
  // Point to a variable holding a heap value (&String)
  let r3 = &s;
  /* Still with reference r3 to be used, Error: cannot move out of `s` because it is borrowed
  let s1 = s;
  */
  // Point to a static string value (&str)
  /* Variables being mutable means their values can be changed, while references being mutable means
   * they should point to mutable data and can change its value. Static values aren't mutable.
   * Error: expected `&mut str`, found `&'static str`
  let r4: &mut str = "this world ";
  */
  let r4 = "this world ";
  // Point to a static string value (&str)
  let r5 = &r4[5..10];
  //Hello, world!
  println!("{r3}, {r5}!");

  let mut a = [1, 1];
  // Point to a stack array value (slice) (&mut [i32])
  let r6 = &mut a[..1];
  r6[0] = 0;
  // Point to a stack variable (&[i32; 2])
  let r7 = &a;
  // Point to a stack array value (&mut [i32; 2])
  let r8 = &mut [2, 2];
  // Point to a stack array value (slice) (&mut [i32])
  let r9 = &mut r8[1..];
  r9[0] = 3;
  //[0, 1], [2, 3]
  println!("{:?}, {:?}", r7, r8);
}

fn reborrow() {
  let mut i = 5;

  // Immutable references
  let r1 = &i;
  // Copy
  let r2 = r1;
  /* Error: cannot assign to `i` because it is borrowed
  i = 6;
  */
  //5, 5
  println!("{r1}, {r2}");
  // Simultaneous immutable references with immutable reborrows
  //5, 5, 5
  println!("{r1}, {r2}, {}", &*r2);
  /* Error: cannot borrow *r2 as mutable, as it is behind a `&` reference
  println!("{}", &mut *r2);
  */

  // Mutable references
  let r3 = &mut i;
  /* If r2 is to be used here, that mutable borrow r3 will fail with Error: cannot borrow `i` as
   * mutable because it is also borrowed as immutable.
  println!("{r2}");
  */
  // Move: r3 is invalidated as the original.
  let r4 = r3;
  /* r3 is invalid. Error: borrow of moved value: r3
  println!("{r3}");
  */
  *r4 = 6;
  //6
  println!("{r4}");

  // Immutable reborrow: Mutation is blocked.
  let r5 = &*r4;
  /* Error: cannot assign to *r4 because it is borrowed
   *r4 = 7;
   */
  //6, 6
  println!("{r4}, {r5}");
  // Mutable reborrow: r4 is blocked as the original.
  /* Equivalent to a implicit reborrow via the typed annotation
  let r6: &mut i32 = r4;
  */
  let r6 = &mut *r4;
  /* Error: cannot borrow r4 as immutable because it is also borrowed as mutable
  println!("{r4}");
  */
  /* Error: cannot assign to *r4 because it is borrowed
   *r4 = 7;
   */
  *r6 = 7;
  //7
  println!("{r6}");
  // Reborrows r5 and r6 are no longer used, and original r4 is resumed.
  *r4 = 8;
  //8
  println!("{r4}");

  let mut guess = String::new();
  // Implicit reborrow for passing params
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  // Resumed
  println!("{guess}");
}
