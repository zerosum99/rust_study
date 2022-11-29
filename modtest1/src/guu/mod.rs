pub mod mod1;

pub fn public_quu() {
    println!("public_quu!")
}

use super::foo;               // 같은 영역의 다른 모듈을 사용
pub fn public_deep_quu() {
    foo::public_foo();        // 같은 영역의 상위 모듈위 함수를 호출
}
