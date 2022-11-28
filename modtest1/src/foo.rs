pub fn public_foo() {
    println!("foo::public_foo!")
}

fn private_foo() {
    println!("foo::private_foo!")
}

mod private_bar {
    pub fn public_bar() {
        println!("foo::private_bar::public_bar!")
    }
}

pub mod bar {
    pub fn public_bar() {
        println!("foo::bar::public_bar!")
    }
}
