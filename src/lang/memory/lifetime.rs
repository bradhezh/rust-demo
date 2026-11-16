use std::io::{self, Read};

pub fn run() {
  borrow_checker();
  lifetime();
  struct_ref_lifetime();
}

fn borrow_checker() {
  // 1. The elem type is inferred as integers, which can default to i32.
  let mut opt = [0; 1];
  io::stdin()
    // 2. Then determined as u8 for the demand. Note that type inference might not be entirely
    // determined at declarations.
    .read_exact(&mut opt)
    .expect("Failed to read");
  let opt = match opt[0] {
    b'y' | b'Y' => true,
    _ => false,
  };

  let s = String::from("hello");
  // Reference to `s` or "world" depending on runtime, while the borrow checker conservatively
  // compares `r` with `s`.
  let r = if opt { &s } else { "world" };
  /* Error: cannot move out of `s` because it is borrowed
  let s1 = s;
  */
  println!("{r}");
}

/* Error: missing lifetime specifier; this function’s return type contains a borrowed value, but the
 * signature does not say whether it is borrowed from `x` or `y`
fn longest(x: &str, y: &str) -> &str {
*/
// The output is treated as both `x` and `y` for lifetime checking, meaning the output lifetime
// should be contained within the smaller lifetime of the data referenced by `x` and the data
// referenced by `y`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

/* Auto-deduced as `fn new_ref<'a>(input: &'a String) -> &'a str`, meaning the output is treated as
 * `input` for lifetime checking
fn new_ref(input: &String) -> &str {
*/
fn new_ref(input: &String) -> &'static str {
  println!("{input}");
  "hello"
}

fn lifetime() {
  let s1 = String::from("longest");
  let result;
  /* If s2 is within this block scope, the function call will fail with Error: s2 does not live long
   * enough.
  {
  */
  let s2 = String::from("a");
  // `result` references "longest" at runtime, while the borrow checker compares it with both s1 and
  // s2 based on the lifetime relationship from the function signature.
  result = longest(&s1, &s2);
  /*
  }
  */
  //longest
  println!("{result}");

  //longest
  let r = new_ref(&s1);
  // With auto-deduction, `r` would be treated as a reference to s1, and this would fail with Error:
  // cannot move out of s1 because it is borrowed.
  let _s1 = s1;
  //hello
  println!("{r}");
}

/* Error: missing lifetime specifier
struct Ref {
  s: &str,
}
*/
#[derive(Debug)]
struct Ref<'a> {
  _s: &'a str,
}

// `_` as the elided lifetime, here is the only one input lifetime.
fn new_ref1(input: &String) -> Ref<'_> {
  Ref { _s: input }
}

fn struct_ref_lifetime() {
  let s = String::from("hello");
  let r = new_ref1(&s);
  /* Error: cannot move out of `s` because it is borrowed
  let s1 = s;
  */
  //Ref { _s: "hello" }
  println!("{r:?}");
}
