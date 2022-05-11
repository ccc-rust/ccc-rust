use sha2::{Digest};

fn main() {
    let nonce = 331742;
    let text = format!("nonce={}\n{}", nonce, "john=>mary:$2.7\ngeorge=>john:$1.3");
    println!("===========text=================\n{}", text);
    let hash = format!("{:x}", sha2::Sha256::digest(text.as_bytes()));
    println!("===========hash=================\n{}", hash);
}
