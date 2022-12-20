

pub fn str_bytes(s : &str) {
    for item in IntoIterator::into_iter(s.bytes()) {
        println!(" item  bytes {} ", item)
    }
}


pub fn str_chars(s : &str) {
    for item in IntoIterator::into_iter(s.chars()) {
        println!(" item chars {} ", item)
    }
}