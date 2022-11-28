mod foo;                             //외부 모듈을 사용
mod guu;                             //외부의 디렉토리의 기본 모듈 사용
use guu::mod1::public_mod1_quu;      //외부 디렉토리 내의 모듈 내의 함수 사용

mod foo1 {                           // 동일한 파일에서 모듈을 정의 , 모듈은 네임스페이스 역할을 함
    pub fn public_foo() {            // 외부에서 사용이 가능하도록 함수 공개
        println!("foo::public_foo!");

        private_foo();               // 비공개는 동일한 모듈 내에서 가능
    }

    fn private_foo() {               // 모듈 내부에서만 사용하는 함수 정의
        println!("foo::private_foo!");
    }

    mod private_bar {                // 모듈 내에 모듈을 정의  모듈은 비공개 처리
        pub fn public_bar() {        // 모듈 외부에서 사용이 가능하도록 회부 공개
            println!("foo::private_bar::public_bar!")
        }
    }

    pub mod bar {                     // 모듈 내에 서브 모듈 정의
        pub fn public_bar() {         // 모듈 외부에서 사용이 가능하도록 함수를 공개
            println!("foo::bar::public_bar!")
        }
    }
}

fn main() {
    foo1::public_foo();                 // 파일 내네 모듈의 공개함수 사용
    // foo::private_foo();  접근제어 비공개

    foo1::bar::public_bar();            // 파일 내의 모듈의 서브 모듈의 공개함수 호출
    foo::public_foo();
    // foo::private_foo();  접근제어 비공개

    foo::bar::public_bar();             // 외부 모듈과 그 내부의 모듈 내의 함수 사용
    guu::public_quu();                  // 디렉토리 정의하고 파일을 정의하면 모듈로 처리됨
    guu::mod1::public_mod1_quu();       // 디렉토리 내의 다른 파일은 별도의 모듈로 인식함

    guu::public_deep_quu()              // 디렉토리 내의 모듈을 인식할 모듈의 함수를 호출
}