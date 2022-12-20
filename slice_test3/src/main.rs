mod str_proc;
mod string_proc;

use crate::str_proc::*;
use crate::string_proc::*;

use std::any::type_name;

fn main() {
    println!("Hello, world! slice test ");

    let s = "String Test";
    str_chars(s);      // 문자로 출력
    str_bytes(s);      // 바이트로 출력

    let ss = String::from("String Test");
    string_chars(&ss);      // 문자로 출력
    string_bytes(&ss);      // 바이트로 출력

    let xx = "300";                    // 일반 변수 지정
    let yy = rtn_borrow(&xx);    // 참조 변수 지정 및 함수 반환값 처리
    println!(" ref {} ", yy);
    println!(" type name {} ", type_of(yy));
    let zz = yy;
    println!(" type name {} ", type_of(zz));
    println!(" ref  * {} ", zz);

    let x = 300;
    let x1 = &x;
    let &x2 = &x;        // 변수 정의할 때 & 붙이면 기존 참조변수가 일반 변수자료형으로 변둉
    println!(" type name 1 {} ", type_of(x1));
    println!(" type name 2 {} ", type_of(x2));

    // let &sss = rtn_borrow("가을 ");  문자열리터럴은 컴파일 에러 발생
    let str1 = String::from("문자열");
    let str2 = &str1;
    let str3 = &str2;    // 이중참조 만들기
    let &str4 = str3;     // 이중참조 중에 하나를 제거
    let str5 = &*str2;    // 참조가 발생하고 있어서 값을 이동할 수 없음 그랟서 다시 참조로 처리
    // 문자열은 이중참조를 제거하려면 에러가 발생한다.  문자열은 소유권 이동이 발생하므로 항상 주의
    let str33 = &&String::from("문자열 이중참조");
    println!(" type name str33 {} ", type_of(str33));
    // 기존 위에 정의된 참조 때문에 본 값을 이동하려면 복사 기능이 없다고 에러
    // which does not implement the `Copy` trait
    // let &&str44 = str33;
    // println!(" type name str44 {} ", type_of(str44));
    //let &&str5 = str3;  두 개를 제거하면 원래 타입 범위를 벗어남 그래서
    //let &str5 : string  = &str1;   이동이 발생하므로 참조타입을 사용할 수 없음
    println!(" type name str2 {} ", type_of(str2));
    println!(" type name str3 {} ", type_of(str3));
    println!(" type name str4 {} ", type_of(str4));
    println!(" type name str5 {} ", type_of(str5));

}


fn rtn_borrow(s : &str) -> &str {
    s
}

// 타입을 출력하는 함수
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
