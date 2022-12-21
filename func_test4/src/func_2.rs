
pub fn rmutate_no_to_zero(param_no:&mut i32){
    *param_no = 0; //de reference
}

pub fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no*0;
    println!("param_no value is :{}",param_no);
}

pub fn ownership() {
    let v = vec![1,2,3];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v;
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap

    // println!("{:?}",v);  소유권이 이동되어 예외 발생
    println!("{:?}",v2);
}