

use typename::TypeName;                // 크레이트 typename도 추가


fn main() {
    println!("Hello, closure test !");

    let mut i = 0;
    let mut f = |x| {
        i += 1;                           // 클로저에서 바인딩할 때 캡처되어 move 설정됨
        println!(" closure i {}", i);
        x
    };
    // i = 1; // compile error
    let a = f(1);
    println!(" f {} ", a);

    call_mytype();
    ref_();

    println!("type {} ", a.type_name_of());  // 타입을 확인할 경우 사용

}

struct MyType;

impl MyType {
    fn doit(&self, _a: u32) {     // 사용하지 않는 식별자에는 언더스코어(_)를 표시하기
        println!(" doit call ");
    }
    fn another(_this: &Self, _a: u32) {  // &Self로 지정한 경우는 메서드로 호출이 불가함
        println!("  another ");          // 반드시 타입명으로 처리해야함   MyType::another(&m, 2);
    }
}

fn call_mytype() {
    let m = MyType;

    // Both can be used as an associated function
    MyType::doit(&m, 1);
    MyType::another(&m, 2);

    // But only `doit` can be used in method position
    m.doit(3);     // OK: `m` is automatically borrowed
    // m.another(4);  // ERROR: no method named `another`
}

fn ref_() {
    let x = 5;

    match x {
        ref r => println!("Got a reference to {:p}", r),
    }

    let mut y = 5;

    match y {
        ref mut mr => println!("Got a mutable reference to {:p}", mr),
    }


    echo("가을이");
    let _s = String::from("겨율이");
    //echo(&s);  //타입 에러 ... 참조자
    // echo(s.as_str());  lifetime을 명기한 경우 단순히 변환해서는 처리가 안됨
}

fn echo(s : &'static str) {   // 생애주기를
    println!(" echo {}", s);
}
