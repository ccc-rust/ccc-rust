use ndarray::{array};

pub fn main() {
    let a=array![1.,2.,3.];
    println!("a = {}", a);
    let b =array![1.,1.,1.];
    println!("a+b={}", &a+&b); // 差異
    println!("a-b={}", &a-&b); // 差異
    println!("a dot b={}", a.dot(&b));
    let m =array![[1.,1.],[2.,2.],[3.,3.]];
    println!("a dot m={}", a.dot(&m));
}
