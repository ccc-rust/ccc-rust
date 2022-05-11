use ndarray::{Array1,Array2};

pub fn gibbs(p:&Array1<f64>, t:&Array2<f64>) {
    let mut p0:Array1<f64> = p.clone();
    println!("p0 = {}", p0);
    let mut p1:Array1<f64>;
    loop {
        p1 = p0.dot(t); // 下一輪的機率分布。
        println!("p1={}", p1);
        let dp = &p1-&p0; // 差異
        let step = dp.dot(&dp).sqrt(); // 差異的大小
        p0 = p1;
        if step < 0.001 { break; } // 假如差異夠小的時候，就可以停止了。
    }
}

