

pub fn largest_gen<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }

    let x = largest;
    x
}

#[derive(Debug)]
pub struct Point<T, U> {
   pub  x: T,
    pub y: U,
}