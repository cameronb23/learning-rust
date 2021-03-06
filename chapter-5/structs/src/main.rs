use std::fmt;

#[derive(Debug)]
struct User {
  id: u8,
  email: String,
  password: String,
  login_count: u32,
}

impl User {
  fn login(&mut self) {
    self.login_count += 1;
  }

  fn from(id: u8, email: &str) -> User {
    User {
      id,
      email: String::from(email),
      password: String::from("some autogenerated pass"),
      login_count: 0,
    }
  }
}

// idk about this yet
// it doesnt seem to work because String is an owned type
// and password/email is both String
impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "User (ID: {} | Email: {})", self.id, self.email)
  }
}

// rgb color!!!!!
struct Color(i32, i32, i32);

fn main() {
  let default_user = User::from(0, "me@cameronb.me");

  let user2 = User {
    id: default_user.id + 1,
    email: String::from("cameron@cameronb.me"),
    ..default_user
  };

  let red_color = Color(255, 0, 0);

  // println!(format!("user: {}", default_user));
}
