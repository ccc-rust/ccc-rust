use ndarray::{array};
mod markov;
mod gibbs;

fn main() {
    let p = array![0.2,0.8];
    let t = array![[0.7, 0.3],[0.5, 0.5]];
    let s = array![1, 0, 1, 1];
    // 序列 s 的馬可夫鏈機率
    let ps = markov::markov(&s, &p, &t);
    println!("markov({})={}", s, ps);
    // 尋找 gibbs 平衡機率
    gibbs::gibbs(&p, &t);
    println!("gibbs 標準答案:P(a)=5/8={} P(b)=3/8={}", 5.0/8.0, 3.0/8.0); // 印出標準答案，以便看看我們找到的答案是否夠接近。
}
