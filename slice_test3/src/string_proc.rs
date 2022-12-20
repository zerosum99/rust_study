


pub fn string_bytes(s : &String) {
    for (i,&item) in s.as_bytes().iter().enumerate() {
        println!(" item  bytes index {} value {} ", i, item)
    }
}


pub fn string_chars(s : &String) {
    for item in s.chars() {
        println!(" item chars {} ", item)
    }
}