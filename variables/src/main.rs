fn main() {
    println!("Hello, world! varialbes ");

    let x = 5;
    println!("불변변수 x의 값 : {} ", x);

    let mut y = 5;
    println!("가변변수 y의 값 : {} ", y);

    y = 6;
    println!("가변변수 y의 값 : {} ", y);

    println!("상수 정의하기 ");

    const XX : i32  = 100;   // 반드시 타입 어노테이션을 정의 , 상수 표현식만 가능
    println!("상수 : {}", XX);


}
