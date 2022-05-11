fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("factorial(3)={}", factorial(3));
    println!("factorial(5)={}", factorial(5));
    println!("factorial(10)={}", factorial(10));
}

