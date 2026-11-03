use std::fmt::format;

// if let은 if와 let을 합쳐 한 pattern과 일치하는 값만 처리하고 나머지는 무시하는 덜 장황한 방법
// if let은 = 기호로 구분한 pattern과 expression을 받고, expression을 match에 넘긴 첫 arm처럼 동작한다.
// if let의 trade-off: 타이핑·들여쓰기·boilerplate가 줄지만 match가 강제하는 exhaustive checking을 잃는다.
// if let에 else를 붙일 수 있다: else 블록은 동등한 match의 _ arm 블록과 같다.
//
// let else: let...else 문법은 if let과 매우 비슷하게 왼쪽에 pattern, 오른쪽에 expression을 받지만, if 분기는 없고 else 분기만 있다.
// -> pattern이 일치하면 pattern의 값을 바깥 scope에 binding한다.
// -> pattern이 일치하지 않으면 프로그램은 else arm으로 흘러가며, 이 else arm은 function에서 return해야 한다.
enum Color {
    RED,
    BLUE,
    GREEN,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    /*
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    */
    // let else
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let color = Color::BLUE;
    if let Color::RED = color {
        println!("color is red");
    } else {
        println!("color is not red");
    }

    let quarter = Coin::Quarter(UsState::Alaska);
    let describe = describe_state_quarter(&quarter).unwrap_or(String::from("oregon"));
    println!("[coin state] -> {describe}");
}
