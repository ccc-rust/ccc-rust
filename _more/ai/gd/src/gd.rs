use ndarray::prelude::*;
const STEP:f64 = 0.01;

// 函數 f 對變數 k 的偏微分: df / dk
pub fn df(f:fn(&Array1<f64>)->f64, p:&Array1<f64>, k:usize)->f64
{
    let mut p1 = p.clone();
    p1[[k]] = p[[k]]+STEP;
    return (f(&p1) - f(p)) / STEP;
}

// 函數 f 在點 p 上的梯度
pub fn grad(f:fn(&Array1<f64>)->f64, p:&Array1<f64>)->Array1<f64>
{
    let mut p1 = p.clone();
    let mut gp = p.clone();
    let plen = p.len();
    for k in 0..plen {
      p1[[k]] = p[[k]]+STEP;
      gp[[k]] = (f(&p1) - f(p)) / STEP;
      p1[[k]] = p[[k]];
    }
    return gp;
}

pub fn norm(a:&Array1<f64>)->f64 {
    return a.dot(&a.t());
}

// 使用梯度下降法尋找函數最低點
pub fn gradient_descendent(f:fn(&Array1<f64>)->f64, p0:&Array1<f64>)->Array1<f64>
{
    let mut p = p0.clone();
    let mut i = 0;
    loop {
        i += 1;
        let fp = f(&p);
        let gp = grad(f, &p); // 計算梯度 gp
        let glen = norm(&gp); // norm = 梯度的長度 (步伐大小)
        println!("{} p={} f(p)={:.3} gp={} glen={:.5}", i, p, fp, gp, glen);
        if glen < 0.00001 {  // 如果步伐已經很小了，那麼就停止吧！
            break;
        }
        let gstep = gp*-1.0*STEP; // gstep = 逆梯度方向的一小步
        p = &p + &gstep; // 向 gstep 方向走一小步
    }
    return p; // 傳回最低點！
}
