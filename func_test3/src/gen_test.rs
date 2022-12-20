// 외부 공개 제너릭 함수 작성
pub fn add<T:std::ops::Add<Output=T>>(x : T, y : T) -> T {
    x + y
}