mod array_slice;


use std::mem;                           // 메모리 모듈 확인
use array_slice::analyze_slice;         // 내부 모듈에 대한 함수 사용

fn main() {
    println!("Hello, world! array slices ");
    // 고정된 크기의 배열 (타입 선언은 불필요하다.)      // 배열 자료형은 대괄호에 타입어노테이션과 길이를 지정
    let xs: [i32; 5] = [1, 2, 3, 4, 5];         // 배열 리터럴은 대괄호로 실제 원소 개수를 정의 정의

    // 모든 요소들이 같은 값으로 초기화 될 수 있다.
    let ys: [i32; 500] = [0; 500];              // 배열의 동일한 원소는 대괄호에 원소값과 세미콜론과 개수 정의

    // 색인은 0부터 시작한다.
    println!(" 색인으로 배열의 첫번째 원소 : {}", xs[0]);
    println!(" 색인으로 배열의 두번째 원소 : {}", xs[1]);

    // `len` 배열의 길이를 반환한다.
    println!(" 배열의 길이 확인  : {}", xs.len());

    // 배열은 스택에 할당된다.
    println!(" 배열의 총 사이즈 확인 : {} bytes", mem::size_of_val(&xs)); // 배열에 대해 대여로 확인

    // 배열은 자동적으로 조각으로 변환하여 대여할 수 있다.
    println!("슬라이스로 배열을 전달 즉 참조로 전달해서 처리 ");
    analyze_slice(&xs);

    // 조각들은 배열의 부분을 가르킬 수 있다.
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 색인이 범위를 넘어가면 panic으로 넘어간다.
    // println!("{}", xs[200]);
}
