#[derive(Debug)] // to pretty-print the struct
pub struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  pub fn new(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
  }

  pub fn area(&self) -> u32 {
    self.width * self.height
  }

  pub fn can_hold(&self, rect: &Rectangle) -> bool {
    self.width > rect.width && self.height > rect.height // implicit returns
  }

  pub fn make_square(size: u32) -> Rectangle {
    Rectangle::new(size, size)
  }
}
