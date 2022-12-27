mod def_trait;
mod def_struct;

//use crate::def_trait::*;  구조체 정의해서 use를 pub으로 처리해서 내보내기 처리
use crate::def_struct::*;

fn main() {
    println!("Hello, world!");
    let p = Person {};
    p.say();
    p.hello();
    println!(" 트레이트 상수 처리 {}", Person::PI);  // 상수는 범위연산자를 통해 조회

    let a = Animal {};
    a.say();
    a.hello();
    println!(" 트레이트 상수 처리 {}", Animal::PI);  // 상수는 범위연산자를 통해 조회

    type Abc=u32;     // 타입별칭 처리
    let a : Abc = 100;
    let c : u32 = a;
    println!(" cccc {} ", c);
}

