mod gd;
use crate::gd::*;
use ndarray::prelude::*;

fn f(p:&Array1<f64>)->f64
{
    let x=p[[0]]; let y = p[[1]];
    return x*x+y*y;
}

fn main() {
    let p = array![1.,2.];
    println!("df(f, {}, 0)={}", p, df(f, &p, 0));
    println!("df(f, {}, 1)={}", p, df(f, &p, 1));
    println!("grad(f, {})={}", p, grad(f, &p));
    let r = gradient_descendent(f, &p);
    println!("r = {}", r);
}

