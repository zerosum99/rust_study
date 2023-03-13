mod add;

use crate::add::sub;

fn main() {
    println!("Hello, test 20230314 !");

    let a = 100;
    println!("정수 입력 = {}",a);

    add::add();      // sub 모듈 호출
    sub::sub();      // sub 모듈 호출

    let x = returns_closure();       // 클로저 반환
    let y = x(100);   // 클로저 실행
    println!(" {} ",y);

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // 함수를 전달해서 처리할 때 : 클로저 사용
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!(" iter {:?} ", list_of_strings);
    // 함수를 전달해서 처리할 때는 : 함수를 제공
    let list_of_numbers1 = vec![11, 12, 13];
    let list_of_strings1: Vec<String> = list_of_numbers1
        .iter()
        .map(ToString::to_string)
        .collect();
    println!(" iter {:?} ", list_of_strings1);

}

// Box로 처리하지 않으면 Sized 트레잇 관련 에러인데, 러스트가 클로저를 저장하는데 필요한 공간을 알 수 없어서
// 생긴 문제로 해석할 수 있다.
// 클로저는 트레잇으로 표현되므로 클로저를 직접 반환할 수 없다.
// 다음과 같이 Box 트레잇 객체를 사용해서 클로저를 반환할 수 있다.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {      // 반환타입은 Box 내의 트레이트 객체로 처리
    Box::new(|x| x + 1)                            // 클로저 반환 , Box를 사용해서 힙에 저장
}


fn add_one(x: i32) -> i32 {
    x + 1
}
// 함수 포인터로 표시하기
// 함수가 함수를 인자로 받으려면 fn 타입을 가진 함수 포인터를 사용한다.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


