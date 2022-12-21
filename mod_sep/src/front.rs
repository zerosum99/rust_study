//pub mod hosting;
// 내부의 모듈을 만들때는 동일한 이름의 디렉토리를 정의해서 해당 모듈을 정의
pub mod hosting;
pub mod mul;

// use 를 외부에 보내기  상대적 경로로 처리
//pub use self::hosting::{add, mul};
// 전체 이름을 지정해서(절대 경로) 외부에 내부 모듈의 함수를 공개하기
pub use crate::front::hosting::{add};
pub use self::mul::{mul};

