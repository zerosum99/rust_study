fn main() {
    println!("Hello, tuple!");

    let x : (u32, char, f32) = (100,'a',200.1);

    println!("Hello, tuple!");
    let (a,b,c) = x;
    println!(" 튜플 분해 : {}", a);
    println!(" 튜플 분해 : {}", b);
    println!(" 튜플 분해 : {}", c);

    println!("Hello, tuple!");
    println!(" 튜플 인덱스 : {}", x.0);
    println!(" 튜플 인덱스 : {}", x.1);
    println!(" 튜플 인덱스 : {}", x.2);


}
