fn main() {
  println!("Hello world");

  let immutable_variable = 12;
  let imut_var_with_type: i32 = 12;

  let mut mutable_variable = 12;

  let my_bool = true;

  let my_char = 'b';

  println!("Print a variable: {}, like this", my_bool);

  let (var_1, var_2) = ("my_var1", 12);

  println!(
    "Print this variable: {0}, again: {0}, and this one: {1}",
    my_char, my_bool
  );

  println!("Format a float to 2 decimal places: {:.2}", 1.234);
  
  println!(
    "Convert to binary: {:b}, hexidecimal: {:x}, octal: {:o}",
    12, 12, 12
  );
  println!("{myArg}", myArg = "Hello");

  println!("abs(-12) = {}", 12i32.abs());
  println!("12^34 = {}", 12i128.pow(34));
  println!("sqrt 12 = {}", 12f32.sqrt());
  println!("cbrt 12 = {}", 12f32.cbrt());
  println!("Round 12.34 = {}", 12.34f32.round());
  println!("Floor 12.34 = {}", 12.34f32.floor());
  println!("Ceiling 12.34 = {}", 12.34f32.ceil());
  println!("e ^ 12 = {}", 12f32.exp());
  println!("log(12) = {}", 12f32.ln());
  println!("12 to Radians = {}", 12f32.to_radians());
  println!("Pi to Degrees = {}", 3.13f32.to_degrees());
  println!("Max 12, 34 = {}", 12i32.max(34i32));
  println!("Min 12, 34 = {}", 12i32.min(34i32));

  let age = 19;
  let _can_vote = if age >= 18 { true } else { false };

  let mut x = 0;

  loop {
    println!("Loop: {}", x);
    if x == 10 {
      break;
    }
    x += 1;
  }

  let mut y = 0;

  while y < 10 {
    println!("While: {}", y);
    y += 1;
  }

  for z in 1..10 {
    println!("For: {}", z);
  }

  let my_string = "This is an example string.";
  println!("Length: {}", my_string.len());

  let (first, second) = my_string.split_at(9);
  println!("First: {}, Second: {}", first, second);

  for c in my_string.chars() {
    println!("{}", c);
  }

  for (i, c) in my_string.chars().enumerate() {
    println!("Index: {}, Character: {}", i, c);
  }

  let split_ws = my_string.split_whitespace();

  for w in split_ws {
    println!("{}", w);
  }

  println!("Find 'example': {}", my_string.contains("example"));  
}