fn main() {
    let number = 6;
    // if expression 코드가 조건에 따라 분기할 수 있게 합니다.
    // if expression의 조건은 반드시 명시적으로 boolean을 사용해야 합니다.
    // 너무 많은 else if식의 사용은 코드를 이해하기 어렵게 하므로, 둘 이상일 경우 코드를 리팩토링하게 될 수도 있습니다.
    // 이런 경우를 위해 match라 불리는 강력한 분기 생성자를 사용합니다.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if가 표현식이기 때문에 let 구문의 우측에 사용할 수 있다.
    let condition = true;
    // 좌측 변수가 가질 수 있는 타입은 하나이므로 if, else if, else block에서 반환하는 값의 타입은 동일해야 한다.
    let num = if condition {
        5
    }
    else {
        6
    };
    println!("The value of number is: {}", num);

    // Rust가 제공하는 세 가지 반복문: loop, while, for
    // `loop` keyword는 Rust에게 그만두라고 명시하여 알려주기 전까지 코드 블럭을 반복 수행한다.(무한 루프)
    // `break` keyword를 위치시켜 프로그램이 언제 루프를 멈춰야 하는지 알려줄 수 있다.
    /*
    loop {
        println!("again!");
    }
    */
    // `while` 조건부 반복
    let mut cnt = 2;
    while cnt != 0 {
        println!("The count is: {cnt}");
        cnt -= 1;
    }
    println!("LIFTOFF!!!");
    // `for` 콜렉션 반복
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {element}");
    }
    println!("LIFTOFF!!!");
    // 정해진 횟수만큼 반복할 때는 `while` 보다 `for`+ `Range` 사용
    // Range는 한 숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성합니다.
    for number in 1..5 {
        println!("The number is {number}");
    }
    println!("LIFTOFF!!!");
    // 마지막 숫자 포함
    for number in 1..=5 {
        println!("The number is {number}");
    }
    println!("LIFTOFF!!!");
    // range를 역순하는 `rev` 메서드
    for number in (1..=4).rev() {
        println!("The number is {number}");
    }
    println!("LIFTOFF!!!");
}