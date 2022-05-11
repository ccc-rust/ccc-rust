fn main() {
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);

  let s1 = "hello"; // s1 的型態會是 &str
  // let s1 = String::from("hello"); // 若改為這行，則會失敗。 (因為非基本型態採用移動語意，所有權會轉移)
  //    |       -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
  let s2 = s1;
  
  println!("s1={}", s1);
  println!("s2={}", s2);
}