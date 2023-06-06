fn main() {

    const VAR : u32 = 100;

    println!("Hello, world!");

    println!(" add {} ", add(100,200));
    println!(" local const {}", VAR);
}


// fn add(x:u32,y:u32) -> u32; 함수 선언을 별도로 지정하지 못함

fn add(x:u32,y:u32) -> u32 {
    x+y
}
