// 의미 있는 그룹을 이루는 관련된 여러 값을 하나로 묶고 이름을 붙이는 custom data type
// struct와 enum이 프로그램의 domain에서 새 type을 만드는 기본 구성 요소
//
// struct와 tuple의 공통점과 차이: struct는 각 data에 이름을 붙이므로 순서에 의존하지 않는다.
// tuple과 달리 struct에서는 각 data 조각에 이름을 붙여 값이 무엇을 뜻하는지 분명하게 한다.
// -> instance의 값을 지정하거나 접근할 때 data의 순서에 의존하지 않아도 되기 때문이다.
//
// struct 정의
// User가 &str 대신 owned type String을 사용한 이유
// -> struct의 각 instance가 자신의 모든 data를 소유하고 그 data가 struct 전체가 유효한 동안 계속 유효하기를 원하기 때문'
// -> struct가 다른 누군가가 소유하는 data의 reference를 저장하는 것도 가능하지만 lifetime을 사용해야 한다.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct에 다른 곳이 소유한 data의 reference를 저장하려면 lifetime이 필요하다.
// username과 email을 String 대신 &str로 선언하려면 구조체에 Lifetime Parameter('a)를 정의해야 한다.
// lifetime 제약 -> 구조체가 빌려온 문자열 데이터가 최소한 해당 참조를 사용하는 동안 유효해야 한다.
// -> 구조체가 빌려온 데이터의 소유자가 먼저 소멸하더라도,
// 해당 데이터를 참조하는 구조체가 이후에 사용되지 않는다면 구조체 변수의 스코프가 더 길 수 있다.
// 이미 존재하는 데이터를 복사하거나 소유권을 이동시키지 않고, 그 데이터의 일부를 참조하면서 여러 작업을 수행하고 싶을 때 사용
// - 문자열 파싱
// - 데이터 분석
// - 큰 데이터의 일부를 참조하는 경우
// - 여러 구조체가 동일한 데이터를 읽어야 하는(공유) 경우
struct UserLifetime<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

struct HttpRequst<'a> {
    method: &'a str,
    path: &'a str,
    version: &'a str,
}

fn print_http_request(request: &HttpRequst) {
    println!(
        "[http request] -> method: {}, path: {}, version: {}",
        request.method, request.path, request.version
    );
}

// field init shorthand: parameter와 field 이름이 같으면 email: email 대신 email만 쓴다.
fn create_user(username: String, email: String, sign_in_count: u64) -> User {
    // struct instance 만들기
    // struct 이름 뒤 중괄호에 key: value 쌍을 쓰며, field 순서는 정의와 달라도 된다.
    User {
        active: true,
        username,
        email,
        sign_in_count,
    }
}

fn print_user(user: &User) -> () {
    println!(
        "[user info] -> active: {}, username: {}, email: {}, sign_in_count: {}",
        user.active, user.username, user.email, user.sign_in_count
    );
}

// tuple struct: struct 이름이 주는 의미는 있지만 field에는 이름 없이 type만 있는 struct
// tuple struct는 struct 이름이 주는 추가적인 의미를 갖지만 field에 연결된 이름은 없고 field의 type만 있다.
// tuple struct가 유용한 두 가지 경우
// case1: tuple 전체에 이름을 붙여 다른 tuple과 다른 type으로 만들고 싶을 때
// case2: 일반 struct처럼 각 field에 이름을 붙이는 것이 장황하거나 불필요할 때
//
// tuple struct 예시
struct Color(u8, u8, u8); // (R, G, B)
struct Point(i32, i32, i32); // (x, y, z)

fn print_color(color: &Color) -> () {
    println!(
        "[color info] -> R: {}, G: {}, B: {}",
        color.0, color.1, color.2
    );
}

fn print_point(point: &Point) -> () {
    println!(
        "[point info] -> x: {}, y: {}, z: {}",
        point.0, point.1, point.2
    );
}

// unit-like struct: field가 하나도 없는 struct로, unit type ()과 비슷하게 동작한다.
// unit-like struct는 어떤 type에 trait을 구현해야 하지만 그 type 자체에 저장할 data는 없을 때 유용할 수 있다.
// Rust에서는 데이터가 없어도 타입 자체에 동작을 구현할 수 있다.
// -> 내부에 데이터가 전혀 없는 구조체라도 트레이트를 구현하여 의미 있는 동작을 제공할 수 있다.
// unit-like struct를 정의할 때는 struct keyword, 원하는 이름, 그리고 세미콜론을 쓴다.
// unit-like struct는 Zero-Sized Type(크기가 0바이트인 타입)
struct UnitStruct;

fn main() {
    let mut user1 = create_user(String::from("user"), String::from("user@example.com"), 1);
    print_user(&user1);
    // dot notation으로 field에 접근하고,
    // mutable instance라면 field에 대입해 값을 바꾼다: mut는 instance 전체에만 붙는다.
    user1.active = false;
    print_user(&user1);

    // struct update syntax
    // struct update syntax는 data를 move한다.
    // user1의 username, email field에 있던 String이 user2로 move 되었으므로 user2를 만든 뒤 user1은 쓸 수 없다.
    let user2 = User {
        active: true,
        ..user1
    };
    print_user(&user2);

    // print_user(&user1); -> 컴파일 에러
    // sign_in_count는 Copy trait을 구현한 타입이기 때문에 사용할 수 있다.
    println!(
        "[user info] -> user1 active: {}, user1 sign_in_count: {}",
        user1.active, user1.sign_in_count
    );

    let white = Color(255, 255, 255);
    print_color(&white);
    let origin = Point(0, 0, 0);
    print_point(&origin);

    // tuple struct도 destructure하고 .index로 접근할 수 있지만, destructure할 때는 struct type 이름을 써야 한다.
    let Color(r, g, b): Color = white;
    println!("[destructure] -> R: {r}, G: {g}, B: {b}");

    // unit struct의 instance는 unit struct 이름으로 만든다.
    let unit_struct = UnitStruct;

    let user_lifetime = UserLifetime {
        active: true,
        username: &user2.username,
        email: &user2.email,
        sign_in_count: 1,
    };

    println!(
        "[user lifetime info] -> active: {}, username: {}, email: {}, sign_in_count:{}",
        user_lifetime.active,
        user_lifetime.username,
        user_lifetime.email,
        user_lifetime.sign_in_count,
    );

    let http_request = String::from("GET /users HTTP/1.1");
    let parts: Vec<&str> = http_request.split_whitespace().collect();

    let request = HttpRequst {
        method: parts[0],
        path: parts[1],
        version: parts[2],
    };

    print_http_request(&request);
}
