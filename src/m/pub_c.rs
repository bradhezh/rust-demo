use super::pri_c;

pub fn foo() {
  println!("Public items under a private ceiling are accessible in its parent tree.");
  pri_c::bar();
}
