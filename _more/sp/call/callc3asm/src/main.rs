extern {
    fn foo();
    fn bar();
    fn add(a:i32, b:i32)->i32;
}

pub fn call() {
    unsafe {
        foo();
        bar();
        let x:i32 = add(3,5);
        println!("add(3,5)={}", x);
    }
}

fn main() {
    call();
    call();
}
