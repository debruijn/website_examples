use fibo_rust::fibo;
use std::time::Instant;


fn main() {
    let before = Instant::now();
    let res = fibo(40);
    let after = Instant::now();
    println!("Result in Rust after {:?} is: {}", after - before, res)
    }
