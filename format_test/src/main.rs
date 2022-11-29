mod format;

fn main() {
    println!("Hello, format_test....");

    format::name_argu();  // 이름인자 출력
    format::base_form();  // 진법별 출력
    format::posit_argu(); // 위치인자 출력
    format::formatting(); // 포매팅 처리
}
