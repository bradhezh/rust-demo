pub fn run() {
  structs();
  struct_ref_lifetime();
  enums();
  options();
}

#[derive(Debug)]
struct Label {
  _text: String,
  width: u32,
  height: u32,
}

// There can be multiple impl blocks for one type.
impl Label {
  // Associated functions. `Self` is an alias of the type.

  fn new(text: &str, width: u32, height: u32) -> Self {
    Self {
      _text: String::from(text),
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
  //Pos(1, 1), Pos(2, 2)
  println!("{pos1:?}, {pos2:?}");

  // Default Move
  let mut label1 = Label::new("Label 1", 100, 10);
  let label2 = Label::new("Label 2", 200, 20);
  //Label { _text: "Label 1", width: 100, height: 10 }
  println!("{label1:?}");
  /* Move: label2 (or label2._text) invalidated, "Label 1" dropped */
  label1 = label2;
  /* Or (Partial Move)
  label1._text = label2._text;
  // Or (Partial Move with Copy for others)
  label1 = Label { ..label2 };
  */
  // label1 is the owner of "Label 2".
  //Label { _text: "Label 2", ...}
  println!("{label1:?}");
  /* label2 (or label2._text) is invalid. Error: borrow of (partially) moved value: label2
  println!("{label2:?}");
  println!("{}", label2._text);
  // Partial Move: Other fields remain valid.
  println!("{}, {}", label2.width, label2.height);
  */

  // Auto-ref/deref for method calls
  //4000, 4000
  println!("{}, {}", label1.size(), Label::size(&label1));
} // label1 drops "Label 2" right before it goes out of scope.

/* Error: missing lifetime specifier
struct Ref {
  s: &str,
}
*/
#[derive(Debug)]
struct Ref<'a> {
  _s: &'a str,
}

// '_ as the elided lifetime, inferred to the same lifetime as the only input.
/*
fn new_ref(s: &mut str) -> Ref<'_> {
*/
fn new_ref(s: &str) -> Ref<'_> {
  Ref { _s: s }
}

fn struct_ref_lifetime() {
  let mut s = String::from("hello");
  let r1 = &mut s;
  /* For lifetime checking, r2 is treated as an immutable/mutable reborrow from r1 accordingly. */
  let r2 = new_ref(r1);
  /* The original blocked
  r1.push_str(", world");
  */
  /* If r1 is reborrowed by the param as a mutable one, this will also be blocked. */
  //hello
  println!("{r1}");
  /* Without Reborrow, this violates simultaneous references rules.
  let r2 = &s;
  */
  //Ref { _s: "hello" }
  println!("{r2:?}");
  // The original resumed
  //hello
  println!("{r1}");
}

#[derive(Debug)]
enum Message<'a> {
  // Unit struct
  Quit,
  // Struct
  Pos { x: i32, y: i32 },
  // Tuple structs
  Text(String),
  MutRef(&'a mut String),
}

impl<'a> Message<'a> {
  fn foo(&mut self) {
    match self {
      Message::Quit => println!("Quit"),
      Message::Pos { x, y } => println!("Pos: {x}, {y}"),
      /* Note that "matching on a reference type with a non-reference pattern implicitly borrows the
       * contents", so don't use `ref` here as Explicit Borrow. */
      Message::Text(s) => println!("Text: {s}"),
      /* Or use a reference pattern without Implicit Borrow, and therefore `ref` should be used here
       * for Explicit Borrow to avoid Move for String.
      &mut Message::Text(ref s) => println!("Text: {s}"),
      */
      /* Note that MutRef(&'a mut String) serves as the pattern source, not the dest slot for `s`
       * here, which still "implicitly borrows the contents", resulting in `&mut &mut String`, which
       * still works here due to auto-deref.
      Message::MutRef(s) => {
      */
      /* Or reference pattern with Explicit Borrow to avoid Move for mutable references, `ref mut`
       * here still results in `&mut &mut String`. */
      &mut Message::MutRef(ref mut s) => {
        s.push_str(", world");
        println!("MutRef: {s}");
      }
    }
  }
}

fn enums() {
  let mut quit = Message::Quit;
  let mut pos = Message::Pos { x: 1, y: 2 };
  let mut text = Message::Text(String::from("hello"));
  let mut s = String::from("hello");
  let mut mut_ref = Message::MutRef(&mut s);
  //Quit
  quit.foo();
  //Pos: 1, 2
  pos.foo();
  //Text: hello
  text.foo();
  //Text("hello")
  println!("{text:?}");
  //MutRef: hello, world
  mut_ref.foo();
  //MutRef("hello, world")
  println!("{mut_ref:?}");
  //hello, world
  println!("{s}");
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
    0 => println!("{x:?} and ZERO"),
    10 => println!("{x:?} and ten"),
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
  let null: Option<i32> = None;
  let five = Some(5);
  let a = Some('a');
  let s = String::from("hello");
  /* Borrow */
  let hello = Some(&s);
  /* Move
  let hello = Some(s);
  */
  // All Copy
  op_on_nullable(null, 0);
  //5 and ZERO
  op_on_nullable(five, 0);
  //'a' and ten
  op_on_nullable(a, 10);
  // Borrow: Copy for Option<&String>, Move for Option<&mut String>
  // Move: Move for Option<String>
  //"hello" and 1
  op_on_nullable(hello, 1);
  //None, Some(5), Some('a')
  println!("{null:?}, {five:?}, {a:?}");
  // Copy: Some("hello"), hello
  /* Move: Error */
  println!("{hello:?}, {s}");
}
