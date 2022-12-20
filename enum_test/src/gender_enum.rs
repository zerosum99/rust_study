


#[derive(Debug)]                 // 이넘 출력할 때 사용하기
pub enum Gender {
    Male,
    Female,
    Other,
}


pub fn gender_match(gender : Gender) {
    println!("===     성별 출력하기     === ");
    match gender {
        Gender::Male => println!("Namja!"),
        Gender::Female => println!("Yeoja!"),
        _ => println!("Molla!")
    }
}