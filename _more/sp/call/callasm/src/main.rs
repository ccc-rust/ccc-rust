extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let sum = unsafe { add(1, 2) };
    println!("{}", sum);
}
