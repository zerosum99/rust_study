// `enum`을 누군가에게 설명하기 위해 만들어보자.
// 주목할 점은, 변수에 이름과 타입 정보를 함께 기재하는 방식이다.:
// `Engineer != Scientist` 그리고 `Height(i32) != Weight(i32)`.
// 각각은 다르고 독립적이다.
pub enum Person {
    // `enum` 는 `unit-like` 일 수 있다,
    Engineer,
    Scientist,
    // 튜플 구조체처럼,
    Height(i32),
    Weight(i32),
    // 혹은 구조체처럼.
    Info { name: String, height: i32 }
}


// `Person` enum을 인자로 받고 아무것도 반환하지 않는 함수.
pub fn inspect(p: Person) {
    // `enum`의 모든 케이스를 만족하는 사용법. (반박불가)
    // 그래서 `match` 를 통해 나눠 살펴보자.
    match p {
        Person::Engineer  => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        // `enum` 내부를 통해 `i` 역구조화.
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // `Info`를 `name` 과 `height`으로 `Info` 역구조화.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}