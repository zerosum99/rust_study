
pub struct Data<T> {
    pub value:T,
}

use crate::gtrait::*;
//declare a structure
pub struct Book {
    pub name:&'static str,
    pub id:u32
}


//implement the trait
impl Printable for Book {
    fn print(&self){
         println!("Printing book with id:{} and name {}",self.id,self.name)
    }
}
