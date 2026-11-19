use std::io;

pub fn run() {
  borrow_checker();
  lifetime();
  generic_ref();
}

fn borrow_checker() {
  let mut opt = String::new();
  io::stdin()
    .read_line(&mut opt)
    .expect("Failed to read line");
  let opt = match opt.trim().to_lowercase().as_str() {
    "y" | "yes" => true,
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

// For lifetime checking, the output should be treated as the input, or an immutable/mutable
// reborrow from the input accordingly if that input is a mutable reference and therefore reborrowed
// by `s` as an immutable/mutable one, no matter whether that output itself is immutable or mutable.
/* Auto-inferred as `fn new_ref<'a>(s: &'a mut str) -> &'a str`
fn new_ref(s: &mut str) -> &str {
*/
/* Auto-inferred as `fn new_ref<'a>(s: &'a str) -> &'a str` */
fn new_ref(s: &str) -> &str {
  s
  /* Note that there's no type specifier for reborrowing, so there's no way to specify return values
   * with reborrowing relationships to params.
  &*s
  */
}

/* Error: missing lifetime specifier; this function’s return type contains a borrowed value, but the
 * signature does not say whether it is borrowed from `x` or `y`
fn longest(x: &str, y: &str) -> &str {
*/
// The output should be treated as both inputs for lifetime checking, so the output lifetime should
// be contained within the smaller lifetime of the data referenced by those inputs.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

fn lifetime() {
  let mut s1 = String::from("hello");
  let r1 = &mut s1;
  /* r1 is a mutable reference and therefore reborrowed by the param as an immutable/mutable one, so
   * for lifetime checking, r2 is treated as an immutable/mutable reborrow from r1 accordingly, even
   * if r2 itself is immutable. */
  let r2 = new_ref(r1);
  /* The original blocked
  r1.push_str(", world");
  */
  /* If r1 is reborrowed by the param as a mutable one, this will also be blocked. */
  //hello
  println!("{r1}");
  /* Without Reborrow, this violates simultaneous references rules.
  let r2 = &s1;
  */
  //hello
  println!("{r2}");
  // The original resumed
  //hello
  println!("{r1}");

  let result;
  /* If s2 is within this block scope, the function call will fail with Error: s2 does not live long
   * enough.
  {
  */
  let s2 = String::from("a");
  // `result` references "hello" at runtime, while the borrow checker compares it with both s1 and
  // s2 based on the lifetime relationship from the function signature.
  result = longest(&s1, &s2);
  /*
  }
  */
  //hello
  println!("{result}");
}

fn mut_ref<T>(r: &mut T) -> &mut T {
  r
}

fn _foo<T>(x: T) -> T {
  x
}

fn generic_ref() {
  let mut s = String::from("hello");
  let r1 = &mut s;
  /* Implicit Reborrow */
  let r2 = mut_ref(r1);
  /* Move for mutable references
  let r2 = _foo(r1);
  */
  /* Lifetimes instantiated within reference types
  let _s = s;
  */
  /* Reborrow: The original blocked
  println!("{r1}");
  */
  r2.push_str(", world");
  //hello, world
  println!("{r2}");
  // Reborrow: The original resumed: hello, world
  /* Move: Error */
  println!("{r1}");
}
