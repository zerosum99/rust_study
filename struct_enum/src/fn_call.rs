
use crate::enum_1::*;

pub fn enum_sex(p : SexCategory) {
    // 이넘 내의 세부적인 값을 추출하기
    match p {
        SexCategory::Name(val)=> {
            println!("{}",val);
        }
        SexCategory::UsrID(val)=> {
            println!("{}",val);
        }
    }
}

pub fn str_add() {
    let n1 = "Tutorials".to_string();
    let n2 = "Point".to_string();

    let n3 = n1 + &n2; // n2 reference is passed 문자열리스트로 처리해야 + 연산 처리
    println!("{}",n3);
}

pub fn for_mat() {
    let n1 = "Tutorials".to_string();
    let n2 = "Point".to_string();
    let n3 = format!("{} {}",n1,n2);
    println!("{}",n3);
}