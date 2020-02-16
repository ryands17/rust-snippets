use std::ops::Mul;
use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
  let r1: Rectangle<f32> = Rectangle {
    width: 20.0,
    height: 25.0,
  };
  let r2: Rectangle<i32> = Rectangle {
    width: 20,
    height: 30,
  };

  println!("********************** Generic Traits **********************");
  println!("Area for r1: {}", r1.area());
  println!("Area for r1: {}", r2.area());
  println!("********************** Generic Traits **********************");

  let numbers = [10, 14, 21, 13, 15, 67, 54, 32];
  let chars = ['a', 'b', 'c', 'f', 'e', 'd', 'g'];

  println!("\n********************** Largest of all **********************");
  println!("Largest among numbers: {}", find_largest(&numbers));
  println!("Largest among characters: {}", find_largest(&chars));
  println!("********************** Largest of all **********************");

  let s1 = "a big large string";
  let s2 = "a small string";
  let t1 = Tweet {
    user: "Ryan".to_string(),
    tweet: "I'm learning Rust...",
  };
  println!("\n********************** Lifetimes **********************");
  println!("The larger of the strings is: {}", compare_and_print_announcement
    (s1, s2, "Comparing strings!"));
  println!("First tweet: {:?}", t1);
  println!("********************** Lifetimes **********************");
}

#[derive(Debug)]
struct Tweet<'a> {
  user: String,
  tweet: &'a str,
}

fn compare_and_print_announcement<'a, T>(s1: &'a str, s2: &'a str,
  text: T) -> &'a str
  where T: Display
{
  println!("~~Text printed: {}~~", text);
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

fn find_largest<T: PartialOrd>(items: &[T]) -> &T {
  let mut largest = &items[0];

  for item in items {
    if item > largest {
      largest = item
    }
  }
  largest
}

struct Rectangle<T> {
  width: T,
  height: T,
}

impl<T> Area<T> for Rectangle<T>
  where T: Copy + Mul<Output=T>
{
  fn area(&self) -> T {
    self.width * self.height
  }
}

trait Area<T> {
  fn area(&self) -> T;
}
