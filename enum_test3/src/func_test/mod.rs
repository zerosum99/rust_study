pub mod sub_func;   // main에서 사용하려면 pub 으로 처리가 필요


pub fn add(x:i32, y:i32) -> i32 {
    //println!(" sub child {} ",sub_func::sub(x,y)); // 하위 모듈의 pub 만 사용이 가능
    // 하위 모듈 내에서 상위 모듈의 함수를 호출하면 순환구조가 생기므로 사용하지 않도록 ...
    x+y
}