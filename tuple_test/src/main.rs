mod tuple_def;

use tuple_def::{Matrix,reverse};

fn main() {
    println!("Hello, world! tuple test ");


    // 서로 다른 타입 무리의 튜플
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // tuple에서 색인으로 값을 추출 할 수 있다.
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 튜플이 튜플의 멤버가 될 수 있다.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 튜플은 출력 가능은 디버그로 처리
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 하나의 요소인 튜플을 만드려면, 괄호와는 별도로 쉼표를 통해 알리는게 필요하다.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 튜플은 바인딩을 생성해서 역구조화 할 수 있다.
    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)
}
