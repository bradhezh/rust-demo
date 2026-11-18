use std::io;

pub fn run() {
  ownership();
  borrow();
  reborrow();
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

fn borrow() {
  let s = String::from("hello");
  // Borrow: Without taking ownership
  refs(&s);
  //hello
  println!("{s}");
}

static G_VAR: i32 = 5;

// The arg can also be &String due to deref coercion.
fn refs(s: &str) {
  // Point to a static variable (&i32)
  let r = &G_VAR;
  //5
  println!("{r}");

  let mut s = String::from(s);
  // Point to a heap string value (&mut str)
  let r1 = &mut s[..1];
  // Auto-ref/deref for method calls
  r1.make_ascii_uppercase();
  // Point to a variable holding a heap value (&String)
  let r1 = &s;
  /* Still with reference r1 to be used, Error: cannot move out of `s` because it is borrowed
  let s1 = s;
  */
  // Point to a static string value (&str)
  /* Variables being mutable means their values can be changed, while references being mutable means
   * they should point to mutable data and can change its value. Static values aren't mutable.
   * Error: expected `&mut str`, found `&'static str`
  let r2: &mut str = "this world ";
  */
  let r2 = "this world ";
  // Point to a static string value (&str)
  let r2 = &r2[5..10];
  //Hello, world!
  println!("{r1}, {r2}!");

  let mut a = [1, 1];
  // Point to a stack array value (slice) (&mut [i32])
  let r1 = &mut a[..1];
  r1[0] = 0;
  // Point to a stack variable (&[i32; 2])
  let r1 = &a;
  // Point to a stack array value (&mut [i32; 4])
  let r2 = &mut [2, 2, 2, 2];
  r2[2] = 3;
  // Point to a stack array value (slice) (&[i32])
  let r2 = &r2[1..3];
  //[0, 1], [2, 3]
  println!("{r1:?}, {r2:?}");
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
  let r1 = &mut i;
  /* If r2 is to be used here, that mutable borrow r1 will fail with Error: cannot borrow `i` as
   * mutable because it is also borrowed as immutable.
  println!("{r2}");
  */
  // Move: r1 is invalidated as the original.
  let r2 = r1;
  *r2 = 6;
  //6
  println!("{r2}");
  // Immutable Reborrow: Mutation is blocked.
  /* Equivalent to implicit reborrowing via the type annotation
  let r3: &i32 = r2;
  */
  let r3 = &*r2;
  /* Error: cannot assign to *r2 because it is borrowed
   *r2 = 7;
   */
  //6, 6
  println!("{r2}, {r3}");
  // Mutable Reborrow: r2 is blocked as the original.
  /* Equivalent to implicit reborrowing via the type annotation
  let r3: &mut i32 = r2;
  */
  let r3 = &mut *r2;
  /* Error: cannot borrow r2 as immutable because it is also borrowed as mutable
  println!("{r2}");
  */
  /* Error: cannot assign to *r2 because it is borrowed
   *r2 = 7;
   */
  *r3 = 7;
  //7
  println!("{r3}");
  // Reborrow r3 is no longer used, and original r2 is resumed.
  *r2 = 8;
  //8
  println!("{r2}");
  /* r1 is invalid. Error: borrow of moved value: r1
  println!("{r1}");
  */

  let mut s = String::new();
  // Implicit reborrowing for passing params
  io::stdin().read_line(&mut s).expect("Failed to read line");
  // Resumed
  println!("{s}");
}
