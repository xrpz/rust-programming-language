/*
unit type -> 원소가 0인 튜플의 타입 -> `()`
unit type의 유일한 값 -> 원소가 0인 튜플 -> `()`
 */
/*
statement -> 어떤 명령들의 나열로 값을 반환하지 않는 어떤 동작을 수행하는 코드
expression -> 평가하면 값이 산출되는 코드
세미콜론(";")이 붙은 표현식은 그 표현식의 결과를 무시(값을 버림)
함수의 본문 `{ ... }`은 block expression
블록의 마지막 expression에 `;`를 붙이면 그 expression은 tail expression이 아니게 되고
별도의 tail expression이 없다면 블록의 결과는 `()`가 된다.
반환 타입을 생략한 함수는 암묵적으로 unit type `()`을 반환한다.
fn print_hello() -> () { println!("hello"); }
== fn print_hello() -> { println!("hello"); }
main 함수의 반환 타입은 Termination trait을 구현한 타입
Rust에서 반환 값은 함수 본문의 마지막 표현식의 값과 동일
*/
fn get_number() -> i32 {
    10
}
fn print_number(arg: i32) {
    println!("The value of arg is: {arg}");
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn multiplied_by_ten(a: i16, b: i16) -> i16 {
    let multiplied_by_ten = { // block은 ten * a * b 값을 산출
        // statement
        let ten = 10;
        // expression
        ten * a * b
    };
    multiplied_by_ten
}
fn main() {
    print_number(100);

    let num = multiply(2, 4);
    println!("num = {num}");

    let result = multiplied_by_ten(5, 7);
    println!("10 * (5 * 7) = {result}");
}