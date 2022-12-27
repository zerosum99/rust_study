//use crate::front::hosting;

pub mod front;
// 다른 모듈로 분리될 경우 상위 모듈에 대한 사용을 지정해야함
// use crate::front; 지정없이 사용가능하려면 front 모듈 내에서 pub use로 공개되는 모듈 등을 지정해야함

fn main() {
    println!("Hello, world!");
    let xx = front::add(200,300);   // 아래의 모듈없이 직접 상위 모듈에서 하위 모듈 내의 함수를 호출함
    println!(" hosting add {}", xx);
    println!("하위모듈로 직접 호출 {} ",front::hosting::add(300,400));

    let yy = front::mul(200,300);
    println!(" hosting mul {}", yy);
    println!("하위모듈로 직접 호출 {} ",front::mul::mul(300,400));
    quux::bar();
    quux::baz();
}

mod quux {                          // 동일한 파일에 지정한 모듈은 use를 사용하지 않아도 됨
    pub use self::foo::{bar, baz};  // 하위 모듈 내의 함수들을 외부에 공개
    pub mod foo {
        pub fn bar() {
            println!("bar");
        }
        pub fn baz() {
            println!("foo");
        }
    }
}