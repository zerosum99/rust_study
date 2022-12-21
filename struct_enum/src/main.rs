// The `derive` attribute automatically creates the
//implementation
// required to make this `enum` printable with
//`fmt::Debug`.

mod enum_1;
mod struct_1;
mod fn_call;

use crate::enum_1::*;
use crate::struct_1::*;
use crate::fn_call::*;


fn main() {
    let p1 = Person {
        name:String::from("Mohtashim"),
        gender:GenderCategory::Male
    };
    let p2 = Person {
        name:String::from("Amy"),
        gender:GenderCategory::Female
    };
    println!("{:?}",p1);
    println!("{:?}",p2);
    println!("name : {}", p1.name);
    println!("gender {:?}", p1.gender);

    println!(" name : {} , gender : {:?}", p2.get_name(), p2.get_gender());
    println!(" name : {} ", p2.get_name());

    let result = is_even(3);
    println!("{:?}",result);
    println!("{:?}",is_even(30));
    println!("{}",is_even(30).unwrap());

    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);

    println!(" even check call : ");
    even_check(100);
    println!(" enum value check call : ");
    let p1 = SexCategory::Name(String::from("Mohtashim"));
    let p2 = SexCategory::UsrID(100);
    println!("{:?}",p1);
    println!("{:?}",p2);
    enum_sex(p1);
    enum_sex(p2);
    // 두 개의 문자열을 붙이기
    str_add();
    // 포맷처리 하기
    for_mat();
}


