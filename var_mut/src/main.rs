fn main() {
    println!("Hello, world!");
    let mut i:i32 = 1;
    let ref_i = &mut i;
    //let another_ref_i = &mut i;  // 가변 참조 빌려주기가 두 번 일어남

    println!(" {}  ", ref_i); //, another_ref_i); 


}
