mod gstruct;
mod gtrait;
mod gfunc;

use gstruct::{Data, Book};      // 구조체 처리
use crate::gtrait::Printable;   // 메서드 작성과 다른 모듈에  지정된 트레이트가 있을 경우 트레이트를 use 처리 필요
use gfunc::print_pro;

fn main() {
    println!("Hello, world!");

    //generic type of i32
    let t:Data<i32> = Data{value:350};
    println!("value is :{} ",t.value);
   //generic type of String
    let t2:Data<String> = Data{value:"Tom".to_string()};
    println!("value is :{} ",t2.value);

    //create an instance of the structure
    let b1 = Book {
         id:1001,
         name:"Rust in Action"
    };
    b1.print();

    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hello TutorialsPoint");

}



