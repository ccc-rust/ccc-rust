// https://github.com/code0100fun/rust-fft/blob/master/src/fft.rs

#![feature(slicing_syntax)]
#![feature(macro_rules)]
extern crate num;
extern crate test;

use num::complex::{Complex};
use std::num::Float;
use std::num::FloatMath;
use std::f64::consts;
use std::rand::{thread_rng, Rng};

#[macro_export]
macro_rules! complex_vec(
    ($($real:expr, $imaginary:expr),*) => ({
        let mut _temp = ::std::vec::Vec::new();
        $(
            let c = Complex::new($real, $imaginary);
            _temp.push(c);
         )*
        _temp
    });
    ($($e:expr),+,) => (vec!($($e),+))
);

pub struct FFT;

impl FFT {
    pub fn new() -> FFT {
        let fft = FFT;
        fft
    }

    pub fn execute_real(&self, vec: Vec<f64>) -> Vec<Complex<f64>> {
        let length = vec.len();
        if length == 1 {
            return complex_vec!(vec[0],0f64);
        }
        let mut i = 0i32;
        let (even, odd): (Vec<f64>,Vec<f64>) = vec.into_iter().partition(|_: &f64| {
            let ret = i % 2 == 0;
            i +=  1;
            return ret;
        });

        let mut fft_even = self.execute_real(even);
        let mut fft_odd = self.execute_real(odd);

        for e in fft_even.clone().iter() {
            fft_even.push(*e);
        }
        for o in fft_odd.clone().iter() {
            fft_odd.push(*o);
        }

        let mut output = Vec::new();

        for i in range(0, length) {
            let c = fft_even[i] + self.omega(-(i as f64), length as f64) * fft_odd[i];
            output.push(c);
        }

        return output;
    }

    pub fn omega(&self, k: f64, n: f64) -> Complex<f64> {
        // e^(i2πk/n) = cos(2πk/n) + sin(2πk/n)i
        let theta = (consts::PI_2 * k) / n;
        Complex::new(FloatMath::cos(theta), FloatMath::sin(theta))
    }
}

fn main() {
  let fx = vec!(
    1f64,
    0f64,
    0f64,
    0f64,
    0f64,
    0f64,
    0f64,
    0f64
  );
  let actual = FFT::new().execute_real(input);
  println!("fft({})={:?}", fx, fq);
}
