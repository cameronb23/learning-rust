// working on ownership concepts

fn main() {
  let mut s = String::from("hi im cameron");

  print_from_ref(&s);

  modify_ref(&mut s);

  // we can do this because we used a ref
  println!("final intro: {}", s);
}

// since this is a reference (&)
// it does not take ownership of the variable
// so it does not get dropped when the function ends
fn print_from_ref(s: &String) {
  println!("{}", s);
}

fn modify_ref(s: &mut String) {
  s.push_str(" and i like cookies");
}
