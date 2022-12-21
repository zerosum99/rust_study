// 구조체 정의

use crate::enum_1::*;  //다른 모듈 사용하기
// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub gender: GenderCategory
}

impl Person {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_gender(&self) -> &GenderCategory {
        &self.gender
    }
}