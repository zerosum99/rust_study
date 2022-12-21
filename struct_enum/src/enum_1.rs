// enum 정의

#[derive(Debug)]
pub enum GenderCategory {
    Male,Female
}

#[derive(Debug)]
pub enum CarType {
    Hatch,
    Sedan,
    SUV
}

pub fn print_size(car:CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("medium sized car");
        },
        CarType::SUV =>{
            println!("Large sized Sports Utility car");
        }
    }
}

pub fn even_check(num:i32) {
    match is_even(num) {
        Some(data) => {
            if data==true {
                println!("Even number");
            }
        },
        None => {
            println!("not even number");
        }
    }
}

pub fn is_even(no:i32)->Option<bool> {
    if no%2 == 0 {
        Some(true)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum SexCategory {
    Name(String),UsrID(i32)
}