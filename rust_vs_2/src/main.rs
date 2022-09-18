fn main() {
    println!("Hello, world!");
    let vs = add();
    print!(" { } ", vs);
}


fn add()-> String {
    "call add !!!".to_string()
}
