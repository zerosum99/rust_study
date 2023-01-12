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
}
