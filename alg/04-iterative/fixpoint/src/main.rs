// 範例 -- 尋找某縮小地圖的不動點。
use ndarray::{array, Array1};
use std::f64::consts::PI;

fn rotate(x:&Array1<f64>, a:f64)->Array1<f64> {
    let rx = array![a.cos()*x[0] - a.sin()*x[1], 
                    a.sin()*x[0] + a.cos()*x[1]];
    return rx;
}

fn translate(x:&Array1<f64>, d:&Array1<f64>)->Array1<f64> {
    return x+d;
}

fn scale(x:&Array1<f64>, s:f64)->Array1<f64> {
    return x*s
}

fn map(x:&Array1<f64>)->Array1<f64> {
    let sx = scale(x, 0.2);
    let tr = array![0.5, 0.5];
    let mx = translate(&sx, &tr);
    let rx = rotate(&mx, PI/4.0);
    return rx
}

fn fixpoint(f:fn(&Array1<f64>)->Array1<f64>, x0:&Array1<f64>, gap:f64)->Array1<f64>
{
    let mut x = x0.clone();
    loop {
        let fx = f(&x);
        println!("x={} fx={}", x, fx);
        let dx = &x-&fx;
        let dist = dx.dot(&dx).sqrt();
        if dist < gap { break; }
        x = fx;
    }
    return x;
}

fn main() {
    let p0 = array![0.1,0.1];
    let fp = fixpoint(map, &p0, 0.0001);
    println!("fp={}", fp);
}

