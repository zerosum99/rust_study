

pub fn iter_call() {
    let list_1 = vec![1,2,3,4];
    let list_iter = list_1.into_iter(); // 데이터가 이동되어 list1을 사용할 수 없음
    // println!(" 벡터 조회 {:?}", list_1);  // value borrowed here after move
    for l in list_iter {
        println!(" iterator element : {}", l);
    }
    let list_2 = vec![1,2,3,4];
    let mut list_iter2 = list_2.iter();    // 데이터 대여만 발생함

    println!(" 데이터의 개수 : {} ", list_2.len());
    println!(" 데이터 처리  1 : {:?} ", list_iter2.next());   // Opetion 자료형의 Some 처리
    println!(" 데이터 처리  2 : {} ", list_iter2.next().unwrap()); // 실제 값은 unwrap으로 처리가 필요
    println!(" 데이터의 개수 : {} ", list_iter2.count());
}