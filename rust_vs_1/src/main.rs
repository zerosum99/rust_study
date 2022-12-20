fn main() {
    println!("Hello world rust !!!");
    let vs =  getNum();
    println!(" func call {} ", vs);

}


fn getNum() -> String {
    let vs = "100";
    vs.to_string()

}cd