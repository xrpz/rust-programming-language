// Module System: 코드를 체계적으로 구성하고, 코드에 접근할 수 있는 범위(Scope)와 공개 여부를 관리하는 시스템
/*
 * 구성 요소        의미
 * Package        여러 Crate를 묶어 빌드·테스트·공유할 수 있게 하는 Cargo 기능 (Cargo.toml 단위)
 * Crate          컴파일러가 한 번에 컴파일하는 최소 단위. binary crate 또는 library crate
 * Module         코드를 그룹화하고 공개 여부(pub)와 접근 범위를 제어 (use로 경로를 스코프에 가져옴)
 * Path           모듈·함수·구조체 등의 위치를 가리키는 이름 (절대: crate::, 상대: self::, super::)
 */
// crate:  Rust compiler가 한 번에 고려하는 가장 작은 코드 단위
// -> crate root: compiler가 시작하는 source file이자 crate의 root module
// -> crate의 두 형태: main이 있는 binary crate, main 없이 기능을 공유하는 library crate
// -> binary crate: command line program이나 server처럼 실행할 수 있는 executable로 컴파일되는 프로그램이다.
// -> binary crate마다 executable이 실행될 때 일어날 일을 정의하는 main function이 반드시 있어야 한다.
// -> library crate: main function이 없고 executable로 컴파일되지도 않는다.
// -> 여러 프로젝트에서 공유할 기능을 정의한다.

// package는 기능 집합을 제공하는 하나 이상의 crate 묶음이며 Cargo.toml을 담는다.
// -> package에는 그 crate들을 어떻게 build할지 기술하는 Cargo.toml file이 있다.
// 예: Cargo 자체도 package인데, 지금까지 코드를 build할 때 써 온 command line 도구의 binary crate를 담고 있다.
// -> Cargo package에는 그 binary crate가 의존하는 library crate도 들어 있다.
// 패키지 규칙: binary crate는 몇 개든, library crate는 최대 하나, library crate든 binary crate든 적어도 crate 하나는 담아야 한다.
// Cargo는 프로젝트 생성 시 Cargo.toml을 만들고, 관례에 따라 src/main.rs를 Binary Crate의 루트로, src/lib.rs를 Library Crate의 루트로 자동 인식해 rustc로 컴파일한다.
// 하나의 Cargo Package에는 src/main.rs(Binary Crate), src/lib.rs(Library Crate), src/bin/의 각 파일(추가 Binary Crate)을 통해 여러 개의 독립적인 Crate를 포함할 수 있다.
// 컴파일은 crate root file에서 시작한다.
// Rust에서 mod 이름;으로 모듈을 선언하면 컴파일러는 인라인 {} 또는 모듈 이름에 해당하는 .rs 파일이나 mod.rs 파일에서 코드를 찾으며, 하위 모듈도 부모 모듈의 디렉터리에서 같은 규칙으로 찾는다.
// Rust에서는 crate::garden::vegetables::Asparagus처럼 Path로 모듈 내부 항목에 접근하며, 기본적으로 private이므로 외부에서 접근하려면 모듈과 해당 항목을 pub으로 공개해야 한다.
// Rust의 use는 긴 Path를 현재 Scope로 가져와, 전체 경로를 반복하지 않고 항목의 이름만으로 접근할 수 있게 하는 기능
// Binary와 Library를 함께 가진 Rust Package에서는 핵심 기능과 모듈을 src/lib.rs에 구현하고,
// src/main.rs는 Package 이름을 경로로 사용해 Library의 공개 API(pub)만 호출하도록 구성하는 것이 권장된다.
//
//
// 함수와 달리 struct, enum 등의 타입은 use로 전체 경로를 가져와 타입 이름만 사용하는 것이 관례적
use std::collections::HashMap;

// 같은 이름의 두 item은 한 scope에 가져올 수 없으므로 fmt::Result와 io::Result처럼 parent module로 구분한다.
use std::fmt;
// use std::io;

// {self, Write}는 io 모듈 자체와 그 안의 Write 트레이트를 동시에 가져오겠다는 의미
use std::io::{self, Write};

fn func1() -> fmt::Result {
    return Result::Ok(());
}

fn func2() -> io::Result<()> {
    return Ok(());
}

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("hello"), 2);
}
