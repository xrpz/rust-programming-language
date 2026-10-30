// &str
// 함수 안에서 출력, 비교, 검색, 길이 확인과 같은 읽기 작업을 한다.
fn print_str(s: &str) -> () {
    println!("print arg: {s}");
}
// &mut str
// 함수 안에서 문자열에 내용을 수정한다.(길이를 변경하지 않음) + 함수가 끝난 뒤 caller가 수정된 문자열을 다시 사용해야한다.
// 소유권은 여전히 caller에게 있고 접근 권한만 빌려준다.(가변 참조 빌림)
// String, 그리고 String의 일부 슬라이스를 받음(&mut String, &mut s[0..5](슬라이스))를 파라미터로 받음
fn upper_str(s: &mut str) {
    s.make_ascii_uppercase();
}

// &mut String
// 함수 안에서 문자열에 내용을 추가(수정)한다. + 함수가 끝난 뒤 caller가 수정된 문자열을 다시 사용해야한다.
// 소유권은 여전히 caller에게 있고 접근 권한만 빌려준다.(가변 참조 빌림)
// String에 대한 참조(&mut String)만 파라미터로 받음
fn append_str(buf: &mut String, msg: &str) -> () {
    buf.push_str(msg);
}

// &mut T
// case A: 원본 데이터를 가변으로 빌리는 경우
// String의 소유저 = a
// String의 소유권 이동 없음
// b는 a에 대한 가변 접근 권한을 가짐
//
// let mut a = String::from("hello");
// let b = &mut a;

// case B: 이미 존재하는 가변 참조를 다른 변수에 대입하는 경우
// String의 소유자 = a
// String의 소유권 이동 없음
// b에서 c로 가변 참조 접근 권한 move (b는 a에 대한 가변 접근 권한을 잃음)
// c는 a에 대한 가변 접근 권한을 가짐
//
// let mut a = 10;
// let b: &mut i32 = &mut a;
//
// let c = b; (b가 빌린 가변 참조 move)
// *b = 30; -> 컴파일 에러

// String
// ReadOnly면서 소유권이 필요한 경우 (소비하거나, 다른 곳에 넘겨 저장하거나(구조체 등), 그대로 반환)
struct User {
    name: String,
}

fn new_user(name: String) -> User {
    User { name }
}
// ReadWrite면서 소유권이 필요한 경우 (수정 후 소비, 수정 후 다른 곳에 넘겨 저장(구조체 등), 수정 후 그대로 반환)
fn normalize_new_user(mut name: String) -> User {
    name.make_ascii_lowercase();
    User { name }
}

fn main() -> () {
    // scope
    {
        let str = String::from("abc");
        println!("{str}");
    }

    let mut a = String::from("hello");
    println!("before append: {a}");
    append_str(&mut a, "world");
    println!("after append: {a}");

    // immutable reference
    let str = String::from("hi!");
    let str2 = &str;
    let str3 = &str;
    println!("str2: {str2}, str3: {str3}");

    // mutable reference
    // 어떤 값에 대한 mutable reference가 있으면 그 값에 대한 다른 reference는 가질 수 없다.
    // -> 위 제약으로인해 Rust는 컴파일 시점에 data race를 막는다.
    let mut _str = String::from("meow");
    let _str2 = &mut _str;
    let _str3 = &mut _str;
    // println!("_str2: {_str2}, _str3: {_str3}"); -> 컴파일 에러

    // 중괄호로 새 scope를 만들면 mutable reference를 여러 개 쓸 수 있다.
    let mut mstr = String::from("mut str");
    {
        let mstr2 = &mut mstr;
        mstr2.replace_range(4.., "str2");
        println!("mstr2: {mstr2}");
    }
    let mstr3 = &mut mstr;
    mstr3.replace_range(4.., "str3");
    println!("mstr3: {mstr3}");
}
