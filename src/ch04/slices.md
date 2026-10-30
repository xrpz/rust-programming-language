# Slices

> 참고: [The Rust Programming Language 4.3 슬라이스 (한국어 2판 번역)](https://rinthel.github.io/rust-lang-book-ko/ch04-03-slices.html), [The Rust Programming Language — The Slice Type (공식 최신판)](https://doc.rust-lang.org/book/ch04-03-slices.html)
---

## 0. 한 줄 요약

**슬라이스는 "원본 데이터의 일부 구간을 가리키는, 복사 없는 읽기 전용 창(view)"이고, 원본이 바뀌어 창이 무효가 되는 상황을 컴파일러가 막아 준다.**

자바로 치면 `List.subList()`처럼 원본을 공유하는 뷰인데, 자바는 원본을 건드리면 **런타임에** `ConcurrentModificationException`이 터지지만 러스트는 **컴파일 단계에서** 그런 코드를 아예 거부합니다.

| 러스트 | 자바에서 가장 가까운 개념 | 차이점 |
|---|---|---|
| `String` | `StringBuilder` (힙에 있고 수정 가능한 문자열) | 러스트는 소유자(owner)가 스코프를 벗어나면 즉시 메모리 해제 (GC 없음) |
| `&str` (문자열 슬라이스) | `CharSequence` 뷰 / `substring()` 결과 | `substring()`은 **새 문자열을 복사**하지만 `&str`은 복사하지 않음 |
| `&[i32]` (배열 슬라이스) | `IntBuffer.wrap(arr, off, len)`, `Arrays.asList(..).subList(..)` | 길이 정보가 타입에 내장, 원본 수명보다 오래 살 수 없음 |
| `"hello"` (문자열 리터럴) | `"hello"` (문자열 상수 풀) | 러스트에서는 타입이 `&str` — 실행 파일 안의 데이터를 가리키는 슬라이스 |

---

## 1. 시작 전 최소 배경지식: 참조와 빌림

슬라이스를 이해하려면 앞 장(4.1 소유권, 4.2 참조와 빌림)의 핵심 두 가지만 알면 됩니다.

**① `&`는 "빌려 보기(참조)"다.**
자바의 모든 객체 변수는 사실상 참조지만, 러스트에서는 값을 직접 갖는 것(`String`)과 빌려 보는 것(`&String`)이 타입으로 구분됩니다. 함수에 `&s`를 넘기면 소유권은 넘어가지 않고 잠시 빌려줄 뿐입니다.

**② 빌림 규칙: "읽기 여러 명" 또는 "쓰기 한 명", 둘 중 하나만.**

```text
같은 시점에
  불변 참조(&T)       여러 개  ── OK
  가변 참조(&mut T)   딱 1개   ── OK
  불변 + 가변 동시에          ── 컴파일 에러
```

자바의 `ReadWriteLock`(읽기 락 여러 개 또는 쓰기 락 하나)을 **컴파일러가 코드 전체에 자동으로 걸어 준다**고 생각하면 비슷합니다. 이 규칙이 슬라이스의 안전성을 만들어 냅니다.

---

## 2. 문제 상황: "첫 번째 단어를 반환하는 함수"

공식 문서가 슬라이스를 소개하려고 드는 예제입니다.

> 공백으로 구분된 문자열을 받아 첫 번째 단어를 반환하라. 공백이 없으면 문자열 전체가 하나의 단어다.

### 2-1. 자바 개발자라면 이렇게 짰을 것이다

```java
// 자바: 단어 끝 인덱스를 반환
static int firstWord(String s) {
    int idx = s.indexOf(' ');
    return idx == -1 ? s.length() : idx;
}
```

자바에서는 그냥 `s.substring(0, idx)`를 반환하면 끝이지만, 이건 새 문자열을 **복사**해서 만드는 방식입니다. 러스트는 "복사 없이 원본의 일부를 가리키는" 방법을 찾고 싶어 합니다. 그런데 슬라이스를 모르면 "문자열의 일부"를 표현할 타입이 없으니, 우선 인덱스를 반환해 봅니다.

### 2-2. 러스트: 인덱스를 반환하는 버전

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();                         // String → 바이트 배열(&[u8])

    for (i, &item) in bytes.iter().enumerate() {      // (인덱스, 바이트) 쌍으로 순회
        if item == b' ' {                             // b' ' = 공백 문자의 바이트 값
            return i;
        }
    }

    s.len()                                           // 세미콜론 없는 마지막 줄 = return
}
```

자바 개발자를 위한 문법 해설:

| 러스트 코드 | 자바로 옮기면 | 설명 |
|---|---|---|
| `s: &String` | `String s` (읽기 전용으로 받음) | 소유권 없이 빌려서 받음 |
| `-> usize` | `int` 반환 | `usize` = 부호 없는 정수, 64비트 OS에서 64비트. 인덱스·길이에 쓰는 표준 타입 |
| `s.as_bytes()` | `s.getBytes(UTF_8)` | 단, 러스트는 **복사 없이** 내부 바이트를 그대로 빌려 줌 |
| `bytes.iter().enumerate()` | `IntStream.range(0, n)` + `bytes[i]` | `(인덱스, &요소)` 튜플을 차례로 돌려줌 |
| `for (i, &item) in ...` | — | 튜플을 패턴으로 분해. 요소가 참조(`&u8`)로 오므로 `&item`으로 받아 값만 꺼냄 |
| `b' '` | `(byte) ' '` | 바이트 리터럴 |
| `s.len()` (마지막 줄) | `return s.length();` | 러스트는 블록의 마지막 표현식이 반환값 |

### 2-3. 이 방식의 치명적 문제: 인덱스와 원본이 따로 논다

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word = 5

    s.clear();                 // s를 ""로 비움

    // word는 여전히 5. 하지만 5가 가리킬 문자열은 이미 사라졌다!
    // 그런데 이 코드는 아무 에러 없이 컴파일된다.
}
```

`word`는 그저 숫자 `5`일 뿐, `s`와 아무 연결이 없습니다. 자바에서도 똑같습니다.

```java
StringBuilder sb = new StringBuilder("hello world");
int word = sb.indexOf(" ");   // 5
sb.setLength(0);              // 비움
sb.substring(0, word);        // 💥 런타임 StringIndexOutOfBoundsException
```

두 번째 단어를 구하는 `second_word`라면 시작·끝 인덱스 두 개(`(usize, usize)`)를 원본과 함께 동기화해야 해서 더 위험해집니다. **"원본 데이터와 묶여 있는 하나의 값"**이 필요합니다. 그것이 슬라이스입니다.

---

## 3. 문자열 슬라이스 `&str`

### 3-1. 문법

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
```

`[시작..끝]`은 **시작 포함, 끝 미포함**입니다. 자바 `substring(begin, end)`와 규칙이 똑같아서 익숙할 겁니다.

| 러스트 | 의미 | 자바 대응 |
|---|---|---|
| `&s[0..5]` | 0 ~ 4 | `s.substring(0, 5)` |
| `&s[..2]` | 처음 ~ 1 (`0..2`와 같음) | `s.substring(0, 2)` |
| `&s[3..]` | 3 ~ 끝 (`3..s.len()`과 같음) | `s.substring(3)` |
| `&s[..]` | 전체 | `s` 자체 |

### 3-2. 메모리 구조: "포인터 + 길이"

슬라이스는 내부적으로 **시작 위치를 가리키는 포인터**와 **길이(끝 − 시작)** 두 값만 갖습니다. 그래서 `&str`의 크기는 64비트 환경에서 항상 16바이트(8 + 8)입니다(`std::mem::size_of::<&str>()` = 16으로 확인).

```text
   스택                               힙
 ┌──── s (String) ────┐          ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
 │ ptr      ──────────┼────────► │ h │ e │ l │ l │ o │   │ w │ o │ r │ l │ d │
 │ len      = 11      │          └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
 │ capacity = 11      │            0   1   2   3   4   5   6   7   8   9  10
 └────────────────────┘                                    ▲
 ┌── world (&str) ────┐                                    │
 │ ptr      ──────────┼────────────────────────────────────┘
 │ len      = 5       │
 └────────────────────┘
```

**자바 `substring()`과의 결정적 차이:** 자바 7u6 이후 `substring()`은 문자 배열을 새로 복사합니다. 러스트 슬라이스는 원본 바이트를 그대로 가리키기만 하므로 문자열 길이와 상관없이 비용이 거의 0입니다.

---

## 4. 슬라이스로 `first_word` 다시 쓰기

### 4-1. 개선된 코드

```rust
fn first_word(s: &String) -> &str {      // 반환 타입이 usize → &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];             // 공백 앞까지 슬라이스
        }
    }

    &s[..]                               // 공백 없으면 전체 슬라이스
}
```

공백을 찾는 로직은 그대로이고, 숫자 대신 **원본에 묶인 슬라이스**를 반환한다는 점만 바뀌었습니다. `second_word`도 `fn second_word(s: &String) -> &str`로 똑같이 만들 수 있고, 시작·끝 인덱스를 따로 관리할 필요가 사라집니다.

### 4-2. 아까의 버그가 이제는 컴파일되지 않는다

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);   // ① s를 불변으로 빌림 (word가 s를 가리킴)

    s.clear();                   // ② s를 가변으로 빌리려 함 → 에러!

    println!("the first word is: {word}");  // ③ 여기서 word(불변 참조)를 아직 사용 중
}
```

실제 컴파일러 출력(rustc 1.97):

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:11:5
   |
10 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
11 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
12 |     println!("the first word is: {word}");
   |                                   ---- immutable borrow later used here
```

에러 메시지 해석:

1. `first_word(&s)` — `word`가 `s`를 **읽기용으로 빌렸다**.
2. `s.clear()` — 문자열을 비우려면 내용을 바꿔야 하니 **쓰기용 빌림(`&mut`)**이 필요하다.
3. `println!`에서 `word`를 쓰므로, ②시점에 읽기 빌림이 **아직 살아 있다**.
4. 빌림 규칙 위반(읽기와 쓰기가 동시에) → 컴파일 실패.

> 💡 반대로 `println!` 줄을 지우면 컴파일됩니다. `word`를 더 이상 안 쓰니 ①의 빌림이 ② 이전에 끝난 것으로 판단하기 때문입니다. 러스트는 "변수의 스코프"가 아니라 "참조가 마지막으로 쓰이는 지점"까지를 빌림 기간으로 봅니다.

### 4-3. 자바와 비교: 같은 버그, 다른 발견 시점

자바의 `subList()`는 러스트 슬라이스와 매우 비슷한 "원본 공유 뷰"입니다. 하지만 안전 장치는 런타임에 동작합니다.

```java
List<Integer> list = new ArrayList<>(List.of(1, 2, 3, 4, 5));
List<Integer> sub = list.subList(1, 3);   // [2, 3] — 원본을 공유하는 뷰

list.clear();                             // 원본을 구조적으로 변경
sub.get(0);                               // 💥 ConcurrentModificationException (런타임)
```

| | 자바 `subList` | 러스트 슬라이스 |
|---|---|---|
| 원본 복사 | 안 함 (뷰) | 안 함 (뷰) |
| 원본 변경 후 뷰 사용 | **런타임** 예외 (운 나쁘면 운영 서버에서 발견) | **컴파일** 에러 (배포 전에 반드시 발견) |
| 검사 비용 | `modCount` 비교로 매 접근마다 런타임 비용 | 0 (컴파일 타임에 끝남) |

공식 문서의 표현대로, 슬라이스는 API를 쓰기 쉽게 만들 뿐 아니라 **이런 종류의 버그 전체를 컴파일 타임에 제거**합니다.

---

## 5. ⚠️ 주의: 슬라이스 인덱스는 "문자"가 아니라 "바이트"

공식 최신판에 있는 중요한 주의사항입니다(한국어 2판 번역본에는 없음). **한국어를 다루는 우리에게 특히 중요**합니다.

- 자바 `String`의 인덱스는 UTF-16 `char` 단위입니다. `"안녕".substring(0, 1)` → `"안"`.
- 러스트 `String`은 UTF-8이고 슬라이스 인덱스는 **바이트** 단위입니다. 한글 한 글자는 UTF-8로 **3바이트**입니다.

```rust
let k = String::from("안녕하세요");
println!("{}", k.len());     // 15  (글자 5개 × 3바이트)
println!("{}", &k[0..3]);    // "안"

let x = &k[0..1];            // 💥 런타임 panic!
```

```text
thread 'main' panicked at src/main.rs:...:
end byte index 1 is not a char boundary; it is inside '안' (bytes 0..3 of string)
```

슬라이스 경계는 반드시 **UTF-8 문자 경계**에 있어야 하고, 글자 중간을 자르면 프로그램이 즉시 종료(panic)됩니다. 이건 컴파일 타임이 아니라 런타임에 검사됩니다(자바의 `StringIndexOutOfBoundsException`과 비슷한 위치).

글자 수 기준으로 자르고 싶으면 `char_indices()`로 바이트 위치를 구하세요.

```rust
let k = String::from("안녕하세요");
// 앞 2글자의 끝 바이트 위치 구하기
let end = k.char_indices().nth(2).map(|(i, _)| i).unwrap_or(k.len());
println!("{}", &k[..end]);   // "안녕"
```

> 이 장의 `first_word` 예제는 ASCII 문자열을 가정합니다. 공백(`b' '`)은 1바이트 문자라 경계 문제가 없어서, 한글 문장 `"안녕 세상"`에 써도 정상 동작합니다. UTF-8 처리는 공식 문서 8.2장에서 자세히 다룹니다.

---

## 6. 문자열 리터럴은 슬라이스다

```rust
let s = "Hello, world!";   // s의 타입은 &str
```

문자열 리터럴은 컴파일된 실행 파일(바이너리) 안에 저장되고, `s`는 **그 바이너리 속 위치를 가리키는 슬라이스**입니다. `&str`이 불변 참조이기 때문에 리터럴도 수정할 수 없습니다.

자바의 문자열 상수 풀에 있는 `"Hello"`가 불변인 것과 결과는 비슷하지만, 이유가 다릅니다.

- 자바: `String` 클래스 자체가 불변(immutable)으로 설계됨
- 러스트: 리터럴의 타입이 **불변 참조(`&str`)**라서 수정 권한이 없음. 수정하고 싶으면 `String::from("...")`으로 소유한 문자열을 만들어야 함

---

## 7. 함수 파라미터는 `&String` 대신 `&str`로

### 7-1. 더 범용적인 시그니처

```rust
fn first_word(s: &String) -> &str {   // 이전: String만 받을 수 있음
fn first_word(s: &str) -> &str {      // 개선: String과 &str 모두 받을 수 있음
```

자바에서 파라미터를 `String`이 아니라 `CharSequence`로 받아서 `String`, `StringBuilder`, `CharBuffer`를 모두 받을 수 있게 하는 것과 같은 설계 원칙입니다. **필요한 것보다 구체적인 타입을 요구하지 말라.** 기능은 그대로이면서 API가 더 일반적이고 유용해집니다.

### 7-2. 어떻게 호출할 수 있나

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // String의 부분 슬라이스 / 전체 슬라이스
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // String의 참조도 OK (전체 슬라이스와 동일하게 취급)
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 문자열 리터럴의 부분 / 전체 슬라이스
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // 리터럴은 이미 &str이므로 그대로 전달 가능
    let word = first_word(my_string_literal);
}
```

`&String`을 `&str` 자리에 그냥 넘길 수 있는 건 **역참조 강제 변환(deref coercion)** 덕분입니다. 컴파일러가 `&String`을 `&str`로 자동 변환해 줍니다. 자바에서 `String`이 `CharSequence`를 구현하니 그 자리에 넘길 수 있는 것과 쓰는 느낌이 비슷합니다(동작 원리는 공식 문서 15.2장에서 다룸).

> 📌 한국어 2판 번역본 예제에는 `first_word(&my_string)` 호출이 없지만, 최신 공식 문서에는 포함되어 있습니다. 실무 러스트 코드에서는 `&my_string[..]`보다 `&my_string`이 훨씬 흔합니다.

---

## 8. 문자열 외의 슬라이스: `&[T]`

슬라이스는 문자열 전용이 아닙니다. 배열·벡터 등 **연속된 메모리에 담긴 모든 컬렉션**의 일부를 가리킬 수 있습니다.

```rust
let a = [1, 2, 3, 4, 5];       // 자바: int[] a = {1, 2, 3, 4, 5};

let slice = &a[1..3];          // 타입: &[i32]

assert_eq!(slice, &[2, 3]);
println!("{}", slice.len());   // 2 — 길이 정보가 슬라이스 안에 있음
```

`&[i32]`도 문자열 슬라이스와 똑같이 **첫 요소 포인터 + 길이**로 동작합니다.

자바와 비교하면:

```java
int[] a = {1, 2, 3, 4, 5};
int[] copy = Arrays.copyOfRange(a, 1, 3);   // 복사본 생성 (원본과 무관)
IntBuffer view = IntBuffer.wrap(a, 1, 2);   // 뷰지만 사용이 번거로움
```

자바에서 배열의 일부를 함수에 넘길 때 흔히 `(int[] arr, int offset, int length)` 세 개를 따로 넘기죠? 러스트에서는 `&[i32]` 하나로 끝나고, 길이를 잘못 넘길 일도 없습니다.

벡터(`Vec<T>`, 자바의 `ArrayList`에 해당)에도 똑같이 쓸 수 있습니다.

```rust
let v = vec![10, 20, 30];
let tail: &[i32] = &v[1..];    // [20, 30]
```

그래서 러스트에서는 "읽기만 하는 함수"의 파라미터를 `&Vec<i32>`가 아니라 `&[i32]`로 받는 것이 관례입니다. 7장의 `&String` → `&str`과 같은 이유입니다.

> 범위를 벗어난 인덱스(`&a[3..10]`)는 컴파일은 되지만 런타임에 panic합니다. 자바의 `ArrayIndexOutOfBoundsException`과 같은 자리라고 보면 됩니다.

---

## 9. 정리

### 핵심 개념

1. **슬라이스 = 원본 일부를 가리키는 참조(포인터 + 길이).** 소유권이 없고, 데이터를 복사하지 않습니다.
2. **문법은 `&x[시작..끝]`**, 끝은 미포함. `..2`, `3..`, `..`으로 생략 가능.
3. **원본과 묶여 있다.** 슬라이스가 살아 있는 동안 원본을 수정하는 코드는 컴파일되지 않습니다(빌림 규칙).
4. **문자열 슬라이스 인덱스는 바이트 단위.** 한글은 3바이트, 글자 중간을 자르면 panic.
5. **`"리터럴"`의 타입은 `&str`.**
6. **파라미터는 `&String` 대신 `&str`, `&Vec<T>` 대신 `&[T]`로** 받아야 범용적입니다.

### 자바 개발자가 자주 하는 실수

| 실수 | 원인 | 해결 |
|---|---|---|
| `&s[0..1]`로 한글 첫 글자를 자르려다 panic | 자바처럼 문자 단위 인덱스라고 착각 | `chars()`, `char_indices()` 사용 |
| 슬라이스를 받아 둔 뒤 원본 `push_str`/`clear` → 컴파일 에러 | 자바처럼 뷰와 원본을 독립적으로 생각 | 슬라이스 사용을 먼저 끝내거나, 필요하면 `.to_string()`으로 복사본 생성 |
| 함수 파라미터를 `&String`으로 선언 | `String`이 기본이라고 생각 | `&str`로 선언 (Clippy도 이를 경고: `ptr_arg`) |
| 슬라이스를 오래 보관하려다 수명(lifetime) 에러 | 슬라이스는 원본보다 오래 살 수 없음 | 소유가 필요하면 `String`/`Vec`으로 복사해 보관 (수명은 공식 문서 10.3장) |

### 한 장 요약 그림

```text
            소유 (Owned)                      빌림 (Borrowed, 슬라이스)
         ─────────────────                 ─────────────────────────────
 문자열        String          ──&s[..]──►          &str
 배열/벡터     [T; N], Vec<T>  ──&v[..]──►          &[T]

 · 소유 타입: 메모리를 관리(해제 책임), 수정 가능(mut일 때)
 · 슬라이스: 포인터+길이, 복사 없음, 원본보다 오래 못 삶, 살아 있는 동안 원본 수정 불가
```

---

## 10. 직접 해보기

1. `fn second_word(s: &str) -> &str`를 구현해 보세요. 단어가 하나뿐이면 빈 문자열 `""`을 반환하도록.
2. 4-2절 코드에서 `println!` 줄을 `s.clear()` **앞**으로 옮기면 컴파일이 되는지, 왜 그런지 설명해 보세요.
3. `fn sum(nums: &[i32]) -> i32`를 만들고, 배열 `[1, 2, 3]`, 벡터 `vec![4, 5, 6]`, 부분 슬라이스 `&v[1..]`를 모두 넘겨 보세요.
4. `"안녕 세상"`에 `first_word`를 호출하면 `"안녕"`이 나오는지 확인하고, 왜 panic이 안 나는지 설명해 보세요.

<details>
<summary>2번, 4번 힌트</summary>

- **2번:** `word`의 마지막 사용이 `s.clear()`보다 앞서므로 불변 빌림이 그 시점에 끝납니다. 가변 빌림과 겹치지 않으니 컴파일됩니다.
- **4번:** 공백 `' '`은 UTF-8에서도 1바이트이고, 공백의 바이트 위치는 항상 글자 경계입니다. 따라서 `&s[0..i]`가 글자 중간을 자르지 않습니다.

</details>

---

## 참고 자료

- 관련 장: 4.2 참조와 빌림 · 8.2 UTF-8 문자열 · 10.3 라이프타임 · 15.2 Deref 강제 변환
