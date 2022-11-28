mod modtest;

use crate::modtest::modtest::connect;

fn main() {
    let mut s = String::from("hello");
    println!(" string {} ", s);

    change(&mut s);

    println!(" string {} ", s);

    let ss = nochange(&s);

    println!(" no change string =>  {:?} ", ss);
    let reference_to_nothing = dangle();

    println!(" no dangle  : {} ", reference_to_nothing);

    let fw  = first_word(&ss);
    println!(" first word  ; {}", fw);

    connect();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s 와 반환값 &String 을 사용하면 댕그링 에러 발생
    s
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn nochange(ss : &String) -> &String {
    ss
}