

// 크레이트 내의 모듈을 사용할 때는 실제 풀이름으로 경로를 상요해서 처리하기


pub fn string_bytes(s : &String) {
    for (i,item) in s.as_bytes().iter().enumerate() {
        println!(" item  bytes {} index {} value {} ",  crate::type_check::type_of(item), i, *item)
    }
}


pub fn string_chars(s : &String) {
    for item in s.chars() {
        println!(" item chars {} ", item)
    }
}