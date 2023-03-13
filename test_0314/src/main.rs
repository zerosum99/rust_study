mod add;

use crate::add::sub;

fn main() {
    println!("Hello, test 20230314 !");

    let a = 100;
    println!("정수 입력 = {}",a);

    add::add();      // sub 모듈 호출
    sub::sub();      // sub 모듈 호출
}

