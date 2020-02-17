use std::collections::HashMap;


fn main() {
  /***************************************** VECTORS *****************************************/
  let numbers = vec![1, 2, 3, 4, 5];
  let mut squares = vec![1, 2, 3, 4, 5];
  let _first = &numbers[0]; // first element reference

  match numbers.get(2) {
    Some(third) => println!("Number is {}", third),
    None => println!("First is {}", numbers.get(10).unwrap_or(&0))
  };

  for i in &mut squares {
    *i = i32::pow(*i, 2);
  }

  /***************************************** HASH MAPS *****************************************/
  let mut scores: HashMap<String, u32> = HashMap::new();
  scores.insert(String::from("blue"), 20);
  scores.insert(String::from("green"), 30);

  let number_squares: HashMap<_, _> = numbers.iter().zip(squares.iter()).collect();
  println!("Team blue: {:?}", scores.get("blue"));

  for (key, value) in &number_squares {
    println!("{}: {}", key, value);
  }

  scores.entry(String::from("yellow")).or_insert(35);
  println!("{:?}", scores);

  let text = "hello world wonderful world";
  let mut word_count = HashMap::new();
  for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
  }

  println!("Words: {:?}", word_count);

  /***************************************** HASH MAPS *****************************************/
  const DATA_VALUES: [i32; 13] = [20, 30, 40, 40, 60, 80, 10, 90, 100, 120,
    110, 80,
    40];
  let length = DATA_VALUES.len();
  DATA_VALUES.sort();
  let sum: i32 = DATA_VALUES.iter().sum();
  println!("Mean is: {}", sum / length as i32);

  let median = if length % 2 == 0 {
    (DATA_VALUES[length / 2] + DATA_VALUES[(length + 1) / 2]) / 2
  } else {
    DATA_VALUES[length / 2]
  };
  println!("Median is: {}", median);

  let mut mode = HashMap::new();
  for num in DATA_VALUES.iter() {
    let count = mode.entry(num).or_insert(0);
    *count += 1;
  }

  let max_value = mode.values().clone().max().unwrap_or(&0);
  for (key, value) in &mode {
    if value == max_value {
      println!("Mode is: {}", key);
      break;
    }
  }
}
