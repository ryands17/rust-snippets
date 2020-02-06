use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdout, stdin, Write};

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("Guess the number!");

  loop {
    print!("Please input your guess: ");
    // required when using print! to flush output
    stdout().flush().unwrap();

    let mut guess = String::new();

    stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u16 = guess.trim().parse().unwrap_or(0);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("That was perfect!");
        break;
      }
    };
  }

  println!("5! = {}", factorial(5));
}

fn factorial(number: u16) -> u64 {
  let mut fact: u64 = 1;
  for n in 2..(number + 1) {
    fact = fact * (n as u64);
  }
  return fact;
}
