
use super::add;

pub fn sub(x:i32, y:i32) -> i32 {
    println!("sub call ");
    sub_test(x,y)
}

fn sub_test(a:i32, b:i32) -> i32 {        // 동일한 모듈에서 사용가능
    println!(" super add {} ", add(a,b));
    println!(" sub test call ");
    a-b
}