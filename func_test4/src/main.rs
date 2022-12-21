

mod func_2;

// 하나의 패키지 내의 모듈은 crate 부터 사용해서 패스를 지정한다.
use crate::func_2::*;
// 모듈 사용하려면 공개된 모듈과 그 내부의 함수 등 사용할 것을 지정해야 한다.
use movies::play;

fn main() {
    let mut rno:i32 = 5;
    rmutate_no_to_zero(&mut rno);
    println!("The value of no is:{}",rno);
    let no:i32 = 5;
    mutate_no_to_zero(no);
    println!("The value of no is:{}",no);

    ownership();

    movies::play("Herold and Kumar".to_string());
    play("공조 2".to_string());

}

// 메인과 같은 파일일 경우는 퍼블릭을 사용하지 않아도 모듈을 접근할 수 있다.
mod movies {
    // 모듈 내의 함수는 항상 pub 을 사용해야 외부에서 사용할 수 있다.
    pub fn play(name:String) {
        println!("Playing movie {}",name);
    }
}



