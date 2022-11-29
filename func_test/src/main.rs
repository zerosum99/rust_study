mod func_mod;

fn main() {
    println!(" Hello, function testing .... ");
    print_number(5);
    let one = add_one(200);
    println!("add one call => {} ", one);

    println!(" function call in module ....");
    func_mod::print_sum(100,200)
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}