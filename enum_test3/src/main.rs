mod book_enum;
mod func_test;

use crate::book_enum::*;
use crate::func_test::add;
use crate::func_test::sub_func::*;

fn main() {
    println!("Hello, world!");
    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    let b3 = Book { isbn: 10, format: BookFormat::Hardback };

    assert!(b1 == b2);
    assert!(b1 != b3);

    let a =add(100,200);
    println!(" add = {} ", a);

    let s = sub(100,80);
    println!(" sub = {} ", s);

    println!(" isbn {} ", b1.get_isbn());
    // 비공개 메서드는 호출할 수 없을  println!(" format {:?} ", b1.get_format());
    println!(" format {:?} ", b1.format);

    let ss = &String::from("abcd");
    let as_ss = ss.as_bytes();
    for (i,item) in as_ss.iter().enumerate() {
        println!(" {} {}", i,item);
    }
}
