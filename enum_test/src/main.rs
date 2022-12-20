// 사용하지 않는 코드 때문에 생성되는 경고를 숨기는 속성.
#![allow(dead_code)]

mod enum_file;
mod gender_enum;
mod literal_operater;                    // 같은 패키지 내의 다른 모듈을 사용하려고 할 때 표시

use enum_file::Person;            // 다른 모듈을 사용하려면 모듈에 있는 구조체/이넘을 가져와야함
use enum_file::inspect;           // 다른 모듈의 함수를 자려옴

use gender_enum::Gender;
use gender_enum::gender_match;
use literal_operater::lit_op;

fn main() {
    println!("Hello, world! enum test ");

    let gender = Gender::Female;

    eprintln!(" 성별 ; {:?}", gender);

    gender_match(gender);


    let gender1 = Gender::Other;
    if let Gender::Other = gender1 {
        println!(" 중성 !");
    }

    let person   = Person::Height(18);
    let amira    = Person::Weight(10);
    // `to_owned()` string 조각에서 독립된 `String`를 만든다.
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca  = Person::Scientist;
    let rohan    = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);

    lit_op();


}


