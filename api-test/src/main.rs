// enable experimental macros
#![feature(proc_macro_hygiene, decl_macro)]

// use Rocket macros
#[macro_use]
extern crate rocket;

// get route macro
#[get("/")]
fn index() -> &'static str {
  "Hello world from Rust!"
}

fn main() {
  // mount the route and launch app
  rocket::ignite().mount("/", routes![index]).launch();
}
