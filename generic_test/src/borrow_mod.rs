
pub fn borrow() {
    let var = 100;
    let &var_a = &var;
    println!(" 대여변수 확인 1 {} ", var_a);
    println!(" 대여변수 확인 2 {} ", &var_a);   // 대여변수에 &를 사용할 수 있다.
    //println!(" 대여변수 확인 3 {} ", *var_a); 정수일 때는 대여변수의 역참조는 불가

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("문자열 대여 : {} ", s2);
    println!("문자열 대여의 역참조 : {} ", *s2);
    println!("문자열 대여의 참조 : {} ", &s2);
    println!("문자열 대여의 대여의 역참조 : {} ", *&s2);
}