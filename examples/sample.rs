use promise::promise;

fn main() {
    let mut v = 3;
    let p: *mut i32 = &mut v;
    println!("Value of v is {}.", promise!(*p));
}
