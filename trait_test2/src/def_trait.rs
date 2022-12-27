
pub trait Say {
    const PI: f32 ;
    fn say(&self);                // 추상 메서드

    fn hello(&self) {             // 구현 메서드
        println!(" ㅎㅎㅎㅎ  ")
    }
}

