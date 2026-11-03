// method: 함수와 비슷하지만 struct(또는 enum, trait object)의 맥락 안에 정의되고 첫 parameter는 항상 self다.
// self: 현재 인스턴스의 소유권을 값으로 전달받는 매개변수로 Copy trait을 구현한 타입이 아니라면 소유권이 이동한다.
// -> 메서드 파라미터에서 self: Self를 self로 줄여쓴다.
// -> self를 써서 instance의 ownership을 가져가는 method는 드물다.
// -> 보통 method가 self를 다른 무언가로 변환하고, 변환 뒤에 호출한 쪽이 원래 instance를 쓰지 못하게 하고 싶을 때 쓰인다고 설명한다.
//
// &self: 현재 인스턴스를 소유권 이동 없이 불변 참조로 빌려오는 매개변수
// -> 메서드 파라미터에서 self: &Self를 &self로 줄여쓴다.
// -> 소유권을 가져가고 싶지 않고, struct의 data를 쓰지 않고 읽기만 하는 메서드에서 사용한다.
//
// &mut self: 현재 인스턴스를 소유권 이동 없이 가변 참조로 빌려오는 매개변수
// -> 메서드 파라미터에서 self: &mut Self를 &mut self로 줄여쓴다.
// -> 소유권을 가져가고 싶지 않지만 메서드를 호출한 struct의 data를 변경하고 싶은 메서드에서 사용한다.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle의 맥락 안에서 함수(메서드)를 정의하려면 Rectangle에 대한 impl(implementation) block을 시작한다.
// impl block 안의 모든 것은 Rectangle type에 연결된다.
// Self: 현재 impl 블록이 대상으로 하는 타입을 나타내는 별칭
// struct 하나에 impl block을 여러 개 둘 수 있다.
// -> 여러 impl block이 유용한 경우는 generic type과 trait을 다루는 경우
impl Rectangle {
    // fn calculate_area(&self) = fn calculate_area(self: &Rectangle)
    fn calculate_area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    fn print(&self) {
        println!(
            "[rectangle info] -> width: {}, height: {}, area: {}",
            self.width,
            self.height,
            self.calculate_area()
        );
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.calculate_area() >= other.calculate_area()
    }

    // impl block안에 정의된 모든 함수는 Associated Function(impl 뒤에 이름이 오는 type에 연결되어 있으므로)
    // impl 안에 정의한 함수 중에서 self를 첫 파라미터로 사용하는 함수는 특별히 Method라고 하고
    // 그렇지 않은 함수는 Method가 아닌 Associated Function
    // -> type의 instance가 없어도 동작하는 함수라면 self를 첫 parameter로 갖지 않는 associated function을 정의할 수 있다.
    // 일반적으로 method가 아닌 associated function은 흔히 constructor로 사용한다.
    // square -> 정사각형을 반환하는 생성자 메서드
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 70);
    rect1.print();
    let rect2 = Rectangle::new(100, 50);
    rect2.print();
    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));

    // associated function은 struct 이름과 :: 문법으로 호출한다.
    let square = Rectangle::square(10);
    square.print();
}
