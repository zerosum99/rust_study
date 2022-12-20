

// 정수와 벡터를 전달해서 최대값을 구하기
pub fn vec_largest(mut largest : i32, number_list: Vec<i32>) {

    for number in &number_list {       // 참조로 가져올 경우
        if *number > largest {               // 비교할 때 역참조로 값을 가져와서 비교
            largest = *number;               // 할당할 경우는 역참조를 통해 값을 조회해서 처리
        }
    }

    println!(" 가장 큰 숫자 1 : {} ", largest);

    let a = largestf(&number_list);
    println!(" 가장 큰 숫자 2 : {} ", a);

    let b = largestf1(&number_list);
    println!(" 가장 큰 숫자 2 : {} ", b);
}


pub fn largestf(list : &[i32]) -> i32 {
    let mut largest = list[0];
    println!(" iterator {:?} ", list.iter());
    for item in list.iter() {
        println!(" item {:?} ", item);
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

pub fn largestf1(list : &[i32]) -> i32 {
    let mut largest = list[0];
    println!(" iterator {:?} ", list.iter());
    for &item in list.iter() {           // 반복자로 가져온 것을 일반 자료형과 비교가 불가
        println!(" item {:?} ", item);        // 그래서 & 기호를 사용해서 값으로 변환
        if item > largest {
            largest = item;
        }
    }
    largest
}
