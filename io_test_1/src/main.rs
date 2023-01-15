use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    //let mut guess = String::new();
    //io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");

    //println!("입력한 값 : {}", guess);

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    println!(" add_one_v1 {} ", add_one_v1(100));
    println!(" add_one_v2 {} ", add_one_v2(100));
    println!(" add_one_v3 {} ", add_one_v3(100));
    println!(" add_one_v4 {} ", add_one_v4(100));
}