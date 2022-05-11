use std::f64::consts::PI;
use num::complex::Complex;

fn complex_list(r:&Vec<f64>, i:&Vec<f64>)->Vec<Complex<f64>> {
    let n = r.len();
    let mut c = Vec::new();
    for t in 0..n {
        c.push(Complex::new(r[t],i[t]));
    }
    return c;
}

fn dft(fx:&Vec<Complex<f64>>)->Vec<Complex<f64>> {
    let n = fx.len();
    let mut fq = vec![Complex::new(0.,0.); n];
    for q in 0..n {
        for x in 0..n {
            let t = (-1.*2.*PI*(q*x) as f64)/n as f64;
            fq[q] += fx[x]*(Complex::new(0.,t)).exp();
        }
    }
    return fq;
}

fn idft(fq:&Vec<Complex<f64>>)->Vec<Complex<f64>> {
    let n = fq.len();
    let mut fx = vec![Complex::new(0.,0.); n];
    for x in 0..n {
        for q in 0..n {
            let t = (2.*PI*(q*x) as f64)/n as f64;
            fx[x] += fq[q]*(Complex::i()*t).exp();
        }
        fx[x] /= n as f64;
    }
    return fx;
}

fn main() {
    let r = vec![1.,0.,1.,0.];
    let i = vec![0.,2.,0.,2.];
    let fx = complex_list(&r, &i);
    let fq = dft(&fx);
    let fx2 = idft(&fq);
    println!("fx={:?}", fx);
    println!("fq={:?}", fq);
    println!("fx2={:?}", fx2);
}
