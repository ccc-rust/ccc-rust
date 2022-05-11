use ndarray::{Array1,Array2};
pub fn markov(s:&Array1<usize>, p:&Array1<f64>, t:&Array2<f64>)->f64{
    let mut r = p[s[0]];
    let slen = s.len();
    for i in 1..slen {
        r = r * t[[s[i-1],s[i]]];
    }
    return r;
}
