mod rectangle;

use rectangle::Rectangle;

fn main() {
  let rect1 = Rectangle::new(40, 50);
  println!("R: {:?}", rect1);

  println!("The area of the rectangle is {} units", rect1.area());

  let rect2 = Rectangle::new(10, 40);
  let rect3 = Rectangle::new(60, 45);

  println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

  let square = Rectangle::make_square(20);
  println!("The area of the square is {} units", square.area());

  let five = Some(5);
  let six = plus_one(five);
  println!("Sum is {}", six.unwrap_or(0));
}

fn plus_one(number: Option<i32>) -> Option<i32> {
  match number {
    None => None,
    Some(n) => Some(n + 1)
  }
}
