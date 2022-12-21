use std::any::type_name;

// 타입을 출력하는 함수
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
