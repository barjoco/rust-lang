use std::io;

fn main() {
  println!("Enter your name: ");

  let mut input = String::new();

  io::stdin().read_line(&mut input)
  .expect("Couldn't read line");

  println!("Hello {}", input);
}