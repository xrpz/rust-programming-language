// match: 값을 일련의 pattern과 비교해 일치하는 pattern의 code를 실행하는 control flow construct
// -> 결과 값(scrutinee 표현식을 평가해서 얻은 값)을 각 arm의 pattern과 순서대로 비교하고, 일치하는 pattern의 code를 실행한다.
// match는 scrutinee 타입이 가질 수 있는 모든 값을 arm들이 빠짐없이 다뤄야 컴파일된다.
// pattern: literal 값, 변수 이름, wildcard 등 많은 것으로 이루어질 수 있다.(=> 의 왼쪽)
// scrutinee 표현식: 패턴 매칭 구문에서 패턴과 비교되는 대상 표현식(패턴이 검사하는 값)
// -> place expression(변수, 필드, 역참조 등): 값을 이동하거나 복사하지 않고 그 메모리 위치를 그대로 검사한다.
// -> value expression(함수 호출 결과 등): 임시값이 만들어져서 그 임시값에 대해 매칭한다.
// -> catch-all pattern: match에서 앞선 패턴과 일치하지 않는 모든 값을 매칭하며, 변수 이름을 사용하면 해당 값을 바인딩할 수 있는 패턴
// -> catch-all arm은 마지막에 두어야 한다
// match arm: arm은 pattern과 code(expression) 두 부분으로 되어 있다.(=> 왼쪽: pattern, => 오른쪽: code)
// -> match는 exhaustive해야 한다.
// -> arm에 있는 expression의 결과 값이 match 표현식 전체가 반환하는 값
// -> match arm의 또 다른 유용한 기능은 pattern과 일치하는 값의 일부에 binding할 수 있다는 것이다.
// -> '_' placeholder: 어떤 값과도 일치하지만 binding하지 않는 특별한 pattern
// -> '_' arm의 code로 unit 값 '()'를 쓰면 '나머지 값은 쓰지 않고 아무 code도 실행하지 않는다'고 명시하는 것이다.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // quarter가 일치하면 state 변수가 그 quarter의 주 값에 binding된다.
        Coin::Quarter(state) => {
            println!("State: {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plsu_two(x: &mut Option<i32>) -> () {
    match x {
        None => {}
        Some(i) => {
            *i += 2;
        }
    }
}

fn is_prime(x: u32) -> bool {
    if (x <= 2) {
        return true;
    }
    for i in 2..=((x as f64).sqrt() as u32) {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let number = Some(10);
    match number {
        // number -> scrutinee 표현식
        Some(value) => println!("{value}"), // arm 1
        None => println!("값 없음"),        // arm 2
    }

    let dime = Coin::Dime;
    let dime_value = value_in_cents(&dime);
    println!("[dime] -> {dime_value}");

    let quarter = Coin::Quarter(UsState::Alaska);
    let quarter_value = value_in_cents(&quarter);
    println!("[quarter] -> {quarter_value}");

    let none = plus_one(None);
    println!("[plus_one result(None)] -> {}", none.unwrap_or(0));

    let num1 = Some(5);
    println!(
        "[plus_one reuslt(Some(5))] -> {}",
        plus_one(num1).unwrap_or(0)
    );

    let mut num2 = Some(10);
    plsu_two(&mut num2);
    println!("[plus_two reuslt(Some(10))] -> {}", num2.unwrap_or(0));

    let val = 10;

    match val {
        num if is_prime(num) => println!("{num} is prime"),
        other => println!("{other} is not prime"),
    }

    let dice_roll = 5;
    match dice_roll {
        3 => println!("3"),
        _ => (),
    }
}
