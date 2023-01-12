use regex::Regex;
//use std::assert;  항상 보장된 매크로

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    assert_call();
    println!(" {:?} ", Color::Red);
    println!(" {:?} ", Color::Green);
    println!(" {:?} ", Color::Blue);

    if Color::Red  as i32 == 0xff0000 {
        println!(" color red 0xff0000 ")
    }

    let x = Color::Red as i32;
    let y = 0xff0000;
    if x == y {
        println!(" color red partialEq oxff0000 ")
    }


}

#[derive(Debug,PartialEq)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

fn assert_call() {
    assert!(4==4);
    assert_eq!(4,4);
    println!("This is never printed.");
}