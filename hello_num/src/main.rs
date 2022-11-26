
use crate::user::add;
mod user;

fn main() {
    println!("Hello, world!");

    println!(" { } ", add(100,200));

    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);

    let mut y = 100;
    println!("mutable value {} ", y);
    y = 999;
    println!("mutable value {} ", y);
}
