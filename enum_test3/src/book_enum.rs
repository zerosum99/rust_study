//use crate::book_enum;

#[derive(Debug)]
pub enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

pub struct Book {
    pub isbn: i32,
    pub format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {         // trait를 구현
        self.isbn == other.isbn
    }
}

impl Book {

    pub fn get_isbn(&self) -> i32 {        // 공개 메서드
        self.isbn
    }
    #[allow(dead_code)]
    fn get_format(&self) -> &BookFormat {   // 비공개 메서드
        &self.format
    }

}