mod movies;

use movies::play;

use movies::english::comedy::play as coplay;

fn main(){
    //short path syntax
    play("Herold and Kumar".to_string());

    coplay("The Hangover......".to_string());

    //full path syntax
    movies::english::comedy::play("Airplane!".to_string());

    let mut v1 = Vec::new();
    v1.push(20);
    v1.push(30);
    v1.push(40);
    println!("size of vector is :{}",v1.len());
    println!("{:?}",v1);
    // 포함관계를 처리할 때는 참조로 값을 전달
    if v1.contains(&20){
        println!("found 20");
    }

    let  mut  v2 = vec![1,2,3];
    println!("{:?}",v2);
    v2.remove(1);
    println!("{:?}",v2);

    for i in &v2 {
        println!("{}",i);
    }

}