use std::io;

fn main() {
  println!("Input any number");

  let mut number = String::new();

  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

  let number: u32 = match number.trim().parse() {
    Ok(num) => num,
    Err(_error) => 0,
  };

  println!("{}", number);

  let result = fibo(number);
  println!("{}", result);
}

fn fibo(x: u32) -> u32 {
  match x {
    0 => 0,
    1 => 1,
    _ => fibo(x - 1) + fibo(x - 2),
  }
}
