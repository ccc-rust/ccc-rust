use rand::Rng;

fn uuid()->String {
    let mut rng = rand::thread_rng();
    let mut id = [0u8; 16];
    for i in 0..16 {
        id[i] = rng.gen();
    }
    return hex::encode(id);
}

fn main() {
    println!("uuid={:?}", uuid());
}
