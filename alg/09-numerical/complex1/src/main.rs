use num::complex::Complex;
fn main() {
    let x = Complex::new(10.0, 20.0); // Must use floats
    let y = Complex::new(3.1, -4.2);
    let sum = x + y;
    println!("Sum: {}", sum);
    println!("x.exp()={}", x.exp());
}
