// Debug는 Rust가 구조체의 필드 이름과 값을 이용해 출력 형태를 자동으로 만들어 줄 수 있다.
// 사용자에게 보여줄 출력은 개발자의 의도에 따라 달라질 수 있기 때문에 Display의 자동 derive를 제공하지 않는다.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_rectangle_area(rect: &Rectangle) -> u64 {
    (rect.width as u64) * (rect.height as u64)
}

fn print_rectangle(rect: &Rectangle, area: u64) -> () {
    println!(
        "[rectangle info] -> width: {}, height: {}, area: {}",
        rect.width, rect.height, area
    );
}
fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    let area = calculate_rectangle_area(&rect);
    print_rectangle(&rect, area);
    println!("[debug] -> {:#?}", rect);

    // Debug 형식으로 값을 출력하는 또 다른 방법은 dbg! macro. -> 전달한 표현식의 결과값을 받아서 출력한 뒤 그대로 반환
    // dbg!는 expression의 ownership을 가져가고(reference를 받는 println!과 대조된다),
    // 코드에서 그 dbg! 호출이 일어난 파일과 줄 번호를 그 expression의 결과값과 함께 출력한 뒤, 값의 ownership을 돌려준다.
    dbg!(&rect);
    // let rec2 = dbg!(rec); 소유권 이동 흐름: rec -> dbg! -> rec2
}
