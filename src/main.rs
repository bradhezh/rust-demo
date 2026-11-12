use std::cmp::Ordering;
use std::io::{self, Read};

use rand::prelude::*;

fn main() {
  guessing_game();
  ownership();
  let s = String::from("hello");
  // Borrow: Without taking ownership
  refs(&s);
  //hello
  println!("{s}");
  reborrow();
  structs();
  enums();
  options();
  borrow_checker();
  lifetime();
  struct_ref_lifetime();
  generic_mut_ref();
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
  println!("{a:?}, {b:?}");
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
  // Auto-ref/deref for method calls
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
  println!("{r7:?}, {r8:?}");
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
  /* Equivalent to a implicit reborrow via the type annotation
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

// Structs follow Move by default unless explicitly implementing Copy. Note that Rust types are
// nominal rather than structural.
#[derive(Debug)]
struct Label {
  // The struct (specifically the field) is the owner.
  text: String,
  width: u32,
  height: u32,
}

// There can be multiple impl blocks for a struct.
impl Label {
  // Associated functions. `Self` is a type alias of the struct.

  fn new(text: &str, width: u32, height: u32) -> Self {
    Self {
      text: String::from(text),
      width,
      height,
    }
  }

  // Methods
  // &self: `self: &Self`
  // &mut self: `self: &mut Self`
  // self: `self: Self`
  fn size(&self) -> u32 {
    // Auto-deref for field access
    self.width * self.height
  }
}

#[derive(Debug, Clone, Copy)]
// Tuple struct
struct Pos(i32, i32);

// Unit-like struct without any field but can implement traits.
struct AlwaysEqual;

fn structs() {
  let _u = AlwaysEqual;

  // Structs implementing Copy
  let pos1 = Pos(1, 1);
  let Pos(_x, _y) = pos1;
  // Copy
  let mut pos2 = pos1;
  pos2.0 = 2;
  pos2.1 = 2;
  //pos1 = Pos(1, 1); pos2 = Pos(2, 2)
  dbg!(pos1, pos2);

  // Default Move
  let mut label1 = Label::new("Label 1", 100, 10);
  let label2 = Label::new("Label 2", dbg!(100 * 2), 20);
  // Fields
  // println! auto-refs params, whereas dbg! not.
  //&label1.text = "Label 1"; label1.width = 100; label1.height = 10
  dbg!(&label1.text, label1.width, label1.height);
  // Move: label2 (or label2.text) invalidated, "Label 1" dropped
  label1 = label2;
  /* Or
  label1.text = label2.text;
  // Or field `text` moved while others copied
  label1 = Label { ..label2 };
  */
  // label1 is the owner of "Label 2".
  //&label1 = Label { text: "Label 2", ...}
  dbg!(&label1);
  /* label2 (or label2.text) is invalid. Error: borrow of (partially) moved value: label2
  dbg!(&label2);
  dbg!(&label2.text);
  // If only partially moved, left fields are still valid.
  dbg!(&label2.width, &label2.height);
  */

  // Auto-ref/deref for method calls
  //label1.size() = 4000; Label::size(&label1) = 4000
  dbg!(label1.size(), Label::size(&label1));
} // label1 drops "Label 2" right before it goes out of scope.

#[derive(Debug)]
enum Message {
  // Unit struct
  Quit,
  // Struct
  Pos { x: i32, y: i32 },
  // Tuple struct
  Write(String),
}

impl Message {
  fn foo(&self) {
    match self {
      Message::Quit => println!("Quit"),
      Message::Pos { x, y } => println!("Pos: {x}, {y}"),
      /* Note that "matching on a reference type with a non-reference pattern implicitly borrows the
       * contents", so don't use `ref` here to explicitly borrow again.
      Message::Write(ref s) => println!("Write: {s}"),
      */
      Message::Write(s) => println!("Write: {s}"),
    }
  }
}

fn enums() {
  let q = Message::Quit;
  let m = Message::Pos { x: 1, y: 2 };
  let w = Message::Write(String::from("hello"));
  //Quit
  q.foo();
  //Pos: 1, 2
  m.foo();
  //Write: hello
  w.foo();
  //w = Write("hello")
  dbg!(w);
}

/*
enum Option<T> {
  None,
  Some(T),
}
*/

fn op_on_nullable<T: std::fmt::Debug>(x: Option<T>, i: i32) {
  /* Equivalent to
  let x = match x {
    Some(v) => v,
    None => return,
  };
  */
  let Some(x) = x else {
    return;
  };
  /* Or
  if let Some(x) = x {
    ...
  }
  */

  // Value-based
  match i {
    3 => println!("{x:?} and three"),
    7 => println!("{x:?} and Seven"),
    /* Equivalent to
    _ => println!("{x:?} and {i}"),
    */
    other => println!("{x:?} and {other}"),
    /* Do nothing
    _ => (),
    */
  }
}

fn options() {
  let five = Some(5);
  let a = Some('a');
  let s = String::from("hello");
  // Reference used to avoid Move for String
  let hello = Some(&s);
  let null: Option<i32> = None;
  // All Copy
  //5 and three
  op_on_nullable(five, 3);
  //'a' and Seven
  op_on_nullable(a, 7);
  //"hello" and 10
  op_on_nullable(hello, 10);
  op_on_nullable(null, 0);
  //five = Some(5); a = Some('a'); &s = "hello"; hello = Some("hello"); null = None
  dbg!(five, a, &s, hello, null);
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

#[derive(Debug)]
enum MutRef<'a, T> {
  Ref(&'a mut T),
}

fn generic_mut_ref() {
  let mut s = String::from("hello");
  let r1 = &mut s;
  /* Implicit reborrow */
  let mut r2 = MutRef::Ref(r1);
  /* Some(T): Move for mutable references
  let mut r2 = Some(r1);
  */
  /* Reborrow: The original blocked
  println!("{r1}");
  */
  match r2 {
    /* MutRef: Note that Ref(&'a mut T) serves as the pattern source, not the dest slot for `r`
     * here, so `ref` is used to avoid Move for mutable references. */
    MutRef::Ref(ref mut r) => r.push_str(", world"),
    /* Some(T)
    Some(ref mut r) => r.push_str(", world"),
    _ => (),
    */
  }
  //MutRef: Ref("hello, world")
  //Some(T): Some("hello, world")
  println!("{r2:?}");
  // Reborrow: The original resumed: hello, world
  /* Move: Error: borrow of moved value: r1 */
  println!("{r1}");
}
