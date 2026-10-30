mod data_types;

// `let` 키워드로 선언한 변수는 기본적으로 immutable
// `mut` 키워드를 붙이면 변경 가능
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; -> ERROR: 불변 변수에 재할당

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is {z}");
    /*
    `mut` 과 shadowing의 차이는 `let` 키워드를 다시 사용하여 효과적으로 새 변수를 선언하고, 값의 유형(type)을 변경할 수 있으면서도 동일 이름을 사용할 수 있다는 점 입니다.
    예를 들어, 공백 문자들을 입력받아 얼마나 많은 공백 문자가 있는지 보여주고자 할 때, 실제로는 저장하고자 하는 것은 공백의 개수일테죠.
    */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {spaces}");
    /*
    shadowing 대신 `mut` 을 사용하는 경우 mismatched type 컴파일 에러 발생
    let mut spaces = "   "; -> type: `&str`
    spaces = spaces.len(); -> type: `usize`
    */
}
/*
대규모 데이터 구조체를 다루는 경우 가변한 인스턴스를 사용하는 것이 새로 인스턴스를 할당하고 반환하는 것보다 빠를 수 있습니다.
데이터 규모가 작을수록 새 인스턴스를 생성하고 함수적 프로그래밍 스타일로 작성하는 것이 더 합리적이고,
그렇기에 약간의 성능 하락을 통해 가독성을 확보할 수 있다면 더 가치있는 선택입니다.
 */
