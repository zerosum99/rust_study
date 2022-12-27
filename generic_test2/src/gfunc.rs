
use std::fmt::Display;

pub fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function : ");
    println!("{}",t);
}