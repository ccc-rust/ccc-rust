#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
  fn new(w:u32, h:u32) -> Self {
      Self {
          width: w,
          height: h,
      }   
  }   

  fn area(&self) -> u32 {
    self.width * self.height
  }  
}

fn main() {
    let r1 = Rectangle::new(30, 50);
    println!("r1.area({:?})={}", r1, r1.area());
}
