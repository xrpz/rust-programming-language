extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::{Rng, RngExt};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        /* ::new에 있는 `::`는 new가 String 타입의 연관함수(정적 메서드) 임을 나타냅니다.
        // 연관함수는 하나의 타입을 위한 함수이며, 이 경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수입니다.
        // new 함수는 새로운 빈 String을 생성합니다.
        // new 함수는 새로운 값을 생성하기 위한 일반적인 이름이므로 많은 타입에서 찾아볼 수 있습니다.
        */
        let mut guess = String::new();
        /* `read_line` 은사용자가 표준 입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요합니다.
        // 그 문자열 인자는 사용자 입력을 추가하면서 변경되므로 가변이어야 합니다.
        */
        /*
        // `&`는 코드의 여러 부분에서 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자 임을 나타냅니다.
        // 참조자는 복잡한 특성으로서 러스트의 큰 이점 중 하나가 참조자를 사용함으로써 얻는 안전성과 용이성입니다.
        */
        /* Result 타입으로 잠재된 실패 다루기
        // read_line은 우리가 인자로 넘긴 문자열에 사용자가 입력을 저장할 뿐 아니라 하나의 값을 돌려 줍니다. 여기서 돌려준 값은 io::Result 입니다.
        // 러스트는 표준 라이브러리에 여러 종류의 Result 타입을 가지고 있습니다. 제네릭 Result이나 io:Result가 그 예시입니다.
        // Result 타입은 열거형(enumerations)로써 enums 라고 부르기도 합니다.
        // 열거형은 정해진 값들만을 가질 수 있으며 이러한 값들은 열거형의 variants 라고 부릅니다.
        // Result의 variants는 Ok와 Err입니다.
        // Ok는 처리가 성공했음을 나타내며 내부적으로 성공적으로 생성된 결과를 가지고 있습니다.
        // Err는 처리가 실패했음을 나타내고 그 이유에 대한 정보를 가지고 있습니다.
        // 이러한 Result는 에러 처리를 위한 정보를 표현하기 위해 사용됩니다.
        // Result 타입의 값들은 다른 타입들처럼 메소드들을 가지고 있습니다.
        // io::Result 인스턴스는 expect 메소드를 가지고 있습니다.
        // 만약 io::Result 인스턴스가 Err일 경우 expect 메소드는 프로그램이 작동을 멈추게 하고 expect에 인자로 넘겼던 메세지를 출력하도록 합니다.
        // 만약 io::Result가 Ok 값이라면 expect는 Ok가 가지고 있는 결과값을 돌려주어 사용할 수 있도록 합니다.
        // 이 경우 결과값은 사용자가 표준 입력으로 입력했던 바이트의 개수입니다.
        */
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        /* parse 메서드는 문자열을 숫자형으로 파싱
        // parse 메소드의 호출은 에러가 발생하기 쉽다. 문자열이 숫자 문자열이 아닌 경우 에러가 발생
        // parse 메서드는 Result 타입을 결과로 돌려 준다.
        */
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        /* Ordering은 Result와 같은 열거형이지만 Ordering의 값은 Less, Greater, Equal입니다.
        // cmp 메소드는 두 값을 비교하며 비교 가능한 모든 것들에 대해 호출할 수 있습니다.
        // 이 메소드는 비교하고 싶은 것들의 참조자를 받습니다.
        // 여기서는 guess와 secret_number를 비교하고 있습니다. cmp는 Ordering 열거형을 돌려줍니다.
        */
        /* match expression을 이용하여 cmp가 guess와 secret_number를 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정할 수 있습니다.
        // match 표현식은 arm 으로 이루어져 있습니다.
        // 하나의 arm은 하나의 패턴 과 match 표현식에서 주어진 값이 패턴과 맞는다면 실행할 코드로 이루어져 있습니다.
        // 러스트는 match에게 주어진 값을 arm의 패턴에 맞는지 순서대로 확인합니다.
        // match 생성자와 패턴들은 개발자의 코드가 마주칠 다양한 상황을 표현할 수 있도록 하고 모든 경우의 수를 처리했음을 확신할 수 있도록 도와주는 강력한 특성입니다.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}