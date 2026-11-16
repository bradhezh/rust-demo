pub fn run() {
  vector();
  string();
  map();
}

fn vector() {
  let v: Vec<i32> = Vec::new();
  //[]
  println!("{v:?}");
  let v = vec![1, 2, 3];
  //[1, 2, 3]
  println!("{v:?}");
  let mut v = Vec::new();
  v.push(0);
  v.push(0);
  //[0, 0]
  println!("{v:?}");

  let Some(r0) = v.get(0) else {
    return;
  };
  //0
  println!("{r0}");
  let Some(r1) = v.get_mut(1) else {
    return;
  };
  *r1 = 1;
  /* Still cause the mutable reference lifetime overlapped with others even though r0 points to a
   * different elem
  println!("{r0:?}");
  */
  //[0, 1]
  println!("{v:?}");

  // Panic for out-of-bounds, auto-deref returned references
  v[0] = v[1];
  //[1, 1]
  println!("{v:?}");

  // Iterate over references `in` a vector reference, whereas iterate over elems `in` a vector. Note
  // that in `for elem in collection`, `collection` is moved due to implicit .into_iter(), and it's
  // also assignment for `elem` to get the value.
  for r in &v {
    //1
    println!("{r}");
  }
  for r in &mut v {
    *r += 1;
  }
  //[2, 2]
  println!("{v:?}");
}

fn string() {
  let data = "initial contents";
  // From the Display trait
  let _s = data.to_string();
  // The method also works on a literal directly:
  let _s = "initial contents".to_string();
  let _s = String::from("initial contents");
  //_s.push('l'); // 4 bytes Unicode as scalar values of `char`
  //_s.push_str("bar");

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  // fn add(self, s: &str) -> String
  let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  //let _s = s1 + "-" + &s2 + "-" + &s3;
  // Auto-refs params like println! but returns String
  let _s = format!("{s1}-{s2}-{s3}");

  /* Error: the type `str` cannot be indexed by `{integer}`
  let h = s[0];
  */
  // Bytes, panic on part of a char's bytes, e.g. Error: end byte index 1 is not a char boundary;
  // it is inside <some-char> (bytes 0..2 of string)
  //let s = &hello[0..1];
  // `char`s
  for c in "Зд".chars() {
    println!("{c}");
  }
  // Bytes
  for b in "Зд".bytes() {
    println!("{b}");
  }
  // Grapheme clusters needs 3rd-party crates, not provided by std.
  //contains, replace
}

fn map() {
  use std::collections::HashMap;
  let mut scores = HashMap::new();
  // Overwrite
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  // The return value of the entry method is an enum called Entry that represents a value that might
  // or might not exist. The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
  scores.entry(String::from("Yellow")).or_insert(50);
  let team_name = String::from("Blue");
  // get method returns an Option<&V>; copied to get an Option<i32> rather than an Option<&i32>;
  // unwrap_or to set score to zero if scores doesn’t have an entry for the key.
  let _score = scores.get(&team_name).copied().unwrap_or(0);
  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  // split_whitespace method returns an iterator over subslices
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
}
