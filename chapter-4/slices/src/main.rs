fn main() {
  let s = String::from("hello world yes");

  let first_word = find_first_word(&s);

  println!("first word: {}", first_word);
}

// this func takes a string slice
// instead of a String so that
// we can use slices and strings on it
fn find_first_word(s: &str) -> &str {
  let s_bytes = s.as_bytes();

  for (index, &item) in s_bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..index];
    }
  }

  &s[..]
}
