fn main() {
    println!("Hello, world!");

    let s = People{};
    s.say();
    People::run();
}

trait Run {
    fn run();              // 연관함수정의 => 구조체가 직접 호출
}
trait Say {
    fn say(&self);        // 메소드 정의 => 인스턴스가 호출
}
// 구조체 정의
struct People {

}
// 메서드 정의
impl Say for People {
    fn say(&self) {
        println!(" 말하다 ")
    }
}
// 연관함수 정의
impl Run for People {
    fn run() {
        println!("달리다")
    }
}


