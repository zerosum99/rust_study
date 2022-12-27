
pub use crate::def_trait::*;   // 트레이트 사용하기

pub struct Animal;           // 구조체를 퍼블릭하면 메서드는 사용가능

impl Say for Animal {        // 트레이트 메서드 적용
    const PI: f32 = 100.33;
    fn say(&self) {
        println!(" bark ");
    }
}

pub struct Person;           // 구조체 공개하면 메서드는 사용가능

impl Say for Person {        // 트레이트 메서드 적용
    const PI: f32 = 200.33;
    fn say(&self) {
        println!(" 말하다 ");
    }
}

