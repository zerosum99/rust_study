

// 튜플은 함수 인자로도 반환 값으로도 사용될 수 있다.
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let`은 튜플의 멤버를 변수에 바인드 할 때 사용된다.
    let (integer, boolean) = pair;       // 튜플의 구조분해

    (boolean, integer)
}

// 다음 구조체는 activity 용.
#[derive(Debug)]
pub struct Matrix(pub f32, pub f32, pub f32, pub f32);    // 튜플도 다른 모듈에서 사용할 때는 모든 필드에 public 처리 필요