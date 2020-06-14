use std::fmt;

struct Number(i32);

struct Person {
  name: String,
  // name: str,
}

mod pedro {
  pub fn hello() {
    println!("Hello from pedro::hello()");
  }
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub fn main() {
  let x = 5 + 90 / 12;
  let person = Person { name: String::from("OK") };
  println!("Hello, x={ok}, {ok} :: 0={p}, 1={:b}", x=x, ok=3, p=person);
  pedro::hello();
}
