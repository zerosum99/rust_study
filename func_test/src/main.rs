mod func_mod;

fn main() {
    println!(" Hello, function testing .... ");
    print_number(5);
    let one = add_one(200);
    println!("add one call => {} ", one);

    println!(" function call in module ....");
    func_mod::print_sum(100,200);

    println!(" add call = {} ", func_mod::add(100,200));

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word = {}",word);
    s.clear()
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}