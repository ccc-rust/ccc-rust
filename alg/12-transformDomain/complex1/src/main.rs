use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let a = num::complex::Complex::new(10.0, 20.0); // Must use floats
    let b = num::complex::Complex::new(3.1, -4.2);
    println!("a+b: {}", a+b);

    let x = Complex::new(0.0, 2.0*PI);
    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
