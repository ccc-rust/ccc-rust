extern {
    fn foo();
    fn bar();
}

pub fn call() {
    unsafe {
        foo();
        bar();
    }
}

fn main() {
    call();
    call();
}
