//extern crate movies_lib;         // 작성된 외부 크레이트를 지정하기
// lib 내부의 별도 모듈 내의 함수 호출
use movies_lib::movies::play;
// lib 모듈 내의 함수 호출
use movies_lib::add;
use std::fs::File;

fn main() {
    println!("inside main of test ");
    play("Tutorial point".to_string());
    movies_lib::movies::play("Tutorial point aaabbb ".to_string());
    println!(" external lib add call {} ", add(100,200));
    println!(" external lib add call {} ", movies_lib::add(300,200));

    let f = File::open("./src/main.rs");   // main.jpg doesn't exist
    match f {
        Ok(f)=> {
            println!("file found {:?}",f);
        },
        Err(e)=> {
            println!("file not found \n{:?}",e);   //handled error
        }
    }

    let result = is_even(10).unwrap();
    println!("result is {}",result);
    let result1 = is_even(11).expect("오류발생 ");
    println!("end of main");
}

fn is_even(no:i32)->Result<bool,String> {
    if no%2==0 {
        return Ok(true);
    } else {
        return Err("NOT_AN_EVEN".to_string());
    }
}