mod largest_mod;
mod borrow_mod;
mod iter_mod;
mod generic_func;

use largest_mod::*;
use borrow_mod::*;
use crate::iter_mod::iter_call;
use crate::generic_func::*;

fn main() {
    println!("Hello, world! generic test ");

    borrow();          // 대여 확인

    iter_call();       // 반복자 처리 호가인

    let number_list = vec![34, 50, 25, 100, 65];
    let largest = number_list[0];
    vec_largest(largest, number_list);     // 벡터에서 큰 수 확인

    let number_list1 = vec![34, 50, 25, 100, 65];
    let gen = largest_gen(&number_list1);
    println!(" generic largest : {} ", gen);

    println!(" 제너릭 처리 확인하기 ");
    // 숫자에 대한 배열처리
    let numbers = vec![34, 50, 25, 100, 65];
    let result = gen_largest(&numbers);
    println!(" 제너릭 최대수 구하기 1 : {}", *result);    // 실제 값으로 처리

    // 문자에 대한 배열 처리
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = gen_largest(&chars);
    println!(" 제너릭 최대수 구하기 2 :  {}", *result);   // 실제 값으로 처리

    println!(" 제너릭으로 구조처 처리 ");
    let both_integer = Point { x: 5, y: 10 };
    println!(" 정수로 구조체 만들기 :  {:?}", both_integer );
}



// 제너릭 함수
// 함수명 다음에 < 사이에 타입매개변수 지정하기
// 타입매개변수는 제한이 가능
// 타입매개변수를 제너릭함수에 매개변수나 반환값에 지정하기
fn gen_largest<T : std::cmp::PartialOrd>(list: &[T]) -> &T {
    // 매개변수는 슬라이스로 받음
    let mut largest = &list[0];
    // 배열의 반복자도 참조 타입으로 처리도미
    for item in list.iter() {
        // 두 참조타임에 대한 비교하기
        if item > largest {
            largest = item;     // 둘 다 참조타입을 위해 변수에 값 할당이 가능함
        }
    }

    largest        // 최종 반환값을 처리 -> 참조타입
}


