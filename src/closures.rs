fn run<F>(f: F)
  where F: Fn()
{
  f();
}

fn main() {
  let print = || println!("Running a closure");
  let add3 = |i: i32| i + 3;
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let greater_than_10: Vec<i32> = numbers.iter().map(|x| x * 2)
    .filter(|x: &i32| x > &10)
    .collect();

  let counter = Counter::new();
  let counter_sum: u32 = counter.zip(Counter::new().skip(1))
    .map(|(a, b)| a + b)
    .filter(|x| x % 3 == 0)
    .sum();

  println!("*************** Closures ***************");
  run(print);
  println!("{}", add3(10));
  println!("{:?}", greater_than_10);
  println!("Counter sum: {}", counter_sum);
  println!("*************** Closures ***************");
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.val += 1;
    if self.val < 6 {
      Some(self.val)
    } else {
      None
    }
  }
}

impl Counter {
  fn new() -> Counter {
    Counter {
      val: 0
    }
  }
}

struct Counter {
  val: u32
}
