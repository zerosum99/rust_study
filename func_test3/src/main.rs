mod gen_test;   // 패키지 내부 모듈 사용

use crate::gen_test::add;   // 패키지 내부 모듈의 공개함수 사용
use num::complex::Complex;  // num 패키지의 complex 모듈의 Complex 타입

// num crate 는 터미털에서 cargo add num 으로 추가
// 이때는 별도의 toml 파일 편집이 필요하지 않음

fn main() {
    println!("Hello, world! function test 3 ");
    let x = 100;
    let y = 300;
    let z = add(x,y);

    println!(" integer add {} ", z);

    let xs = 200.22;
    let ys = 300.33;
    let zs = add(xs,ys);
    println!(" float  add {} ", zs);

    let xc = Complex {re: 200, im: 100 };
    let yc = Complex {re : 200, im : 100 };
    let zc = add(xc,yc);
    println!(" complex  add {} ", zc);

}
