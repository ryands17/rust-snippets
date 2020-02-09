pub fn factorial(number: u16) -> u64 {
  let mut fact: u64 = 1;
  for n in 2..(number + 1) {
    fact = fact * (n as u64);
  }
  return fact;
}
