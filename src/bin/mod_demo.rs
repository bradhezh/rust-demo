// Package name for the lib crate
use rust_demo::m::pub_c;

fn main() {
  println!("Public items with a public chain up to its root are accessible in other crates.");
  pub_c::foo();
}
