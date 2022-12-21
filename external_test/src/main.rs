extern crate movies_lib;         // 작성된 외부 크레이트를 지정하기
// lib 내부의 별도 모듈 내의 함수 호출
use movies_lib::movies::play;
// lib 모듈 내의 함수 호출
use movies_lib::add;

fn main() {
    println!("inside main of test ");
    play("Tutorial point".to_string());
    println!(" external lib add call {} ", add(100,200));
}