fn mpower(a:isize, n:isize, p:isize)->isize {
  let mut r = a;
  for _i in 2..n+1 {
    r = (r * a) % p;
  }
  return r;
}

fn main() {
  let p = 61;
  let q = 53;
  let pq = p*q; // lcm(61,53)=780
  let e = 17;
  let d = 413;
  let m1 = vec![65, 22, 37, 18, 29];
  let mut e1 = Vec::new();
  let mut m2 = Vec::new();
  for m in m1.iter() {
    let c = mpower(*m, e, pq);
    e1.push(c);
    let mx = mpower(c, d, pq);
    m2.push(mx);
  }
  
  println!("m1={:?}", m1);
  println!("e1={:?}", e1);
  println!("m2={:?}", m2);
}
