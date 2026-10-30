# 참조자(References)와 빌림(Borrowing) — 자바 개발자를 위한 러스트 입문

> 참고 자료
> - 한국어 번역본(2판): https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html
> - 공식 문서(최신판): https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
---

## 0. 한 줄 요약

> **참조자(`&T`)는 "소유하지 않고 잠깐 빌려 쓰는" 포인터이고,
> 러스트 컴파일러는 "쓰기 1명 또는 읽기 여러 명" 규칙을 컴파일 타임에 강제해서
> 데이터 레이스와 댕글링 포인터를 원천 차단한다.**

자바 개발자에게 익숙한 말로 바꾸면,
**`ReadWriteLock`을 런타임이 아니라 컴파일러가 대신 걸어주는 것**에 가깝습니다.

---

## 1. 들어가기 전에: 자바와 러스트의 출발점이 다르다

| | 자바 | 러스트 |
|---|---|---|
| 메모리 해제 | GC가 "아무도 안 쓰는 객체"를 찾아서 해제 | **소유자(owner)** 변수가 스코프를 벗어나는 순간 해제 |
| 객체 변수 | 모든 객체 변수는 힙 객체를 가리키는 참조 | 변수가 값을 **소유**함 |
| 대입/전달 | 참조값이 복사되어 여러 변수가 같은 객체를 공유 | 기본적으로 **소유권이 이동(move)** 하고, 원래 변수는 사용 불가 |

앞 절(4.1 소유권)의 핵심만 복습하면 이렇습니다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;              // 소유권이 s1 → s2로 이동(move)
    println!("{s1}");         // ❌ 컴파일 에러: s1은 더 이상 유효하지 않음
}
```

자바라면 `s1`, `s2`가 같은 객체를 함께 가리키겠지만, 러스트에서는 **값의 주인은 항상 한 명**입니다.
주인이 둘이면 "누가 메모리를 해제할지"가 애매해지기 때문입니다(이중 해제 문제).

---

## 2. 문제 상황: 함수에 넘기면 돌려받아야 한다?

소유권 규칙만 있으면 함수 호출이 꽤 불편해집니다.

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1의 소유권이 함수로 넘어감
    println!("'{s2}'의 길이는 {len}");      // 다시 쓰려면 돌려받아야 함
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 소유권을 다시 돌려주려고 튜플로 반환
}
```

길이 하나 구하려고 문자열을 넘겼다가 튜플로 다시 돌려받는 건 누가 봐도 번거롭습니다.
자바였다면 그냥 `calculateLength(s1)` 하고 끝났겠죠.

이 문제를 해결하는 것이 **참조자(reference)** 입니다.

---

## 3. 참조자 `&` — "소유하지 않고 빌린다"

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // s1을 "빌려줌"

    println!("'{s1}'의 길이는 {len}"); // ✅ s1은 여전히 유효
}

fn calculate_length(s: &String) -> usize { // s는 String을 가리키는 참조자
    s.len()
} // s가 스코프를 벗어나지만, 소유자가 아니므로 String은 해제되지 않음
```

- `&s1` : `s1`의 값을 **가리키지만 소유하지는 않는** 참조자를 만듭니다.
- `&String` : 매개변수 타입이 "String의 참조"라는 뜻입니다.
- 함수가 끝나도 **참조자만 사라질 뿐, 원본 `String`은 그대로**입니다.

이렇게 참조자를 만드는 행위를 **빌림(borrowing)** 이라고 부릅니다.
현실에서 남의 물건을 빌리는 것처럼, 빌린 쪽은 주인이 아니고 다 쓰면 돌려줘야 합니다(실제로는 아무 일도 안 해도 됩니다. 소유권을 받은 적이 없으니까요).

### 메모리 구조

```
  스택                                   힙
 ┌───────────┐     ┌──────────────┐     ┌───┬───┬───┬───┬───┐
 │ s (&String)│────▶│ s1 (String)  │────▶│ h │ e │ l │ l │ o │
 └───────────┘     │  ptr         │     └───┴───┴───┴───┴───┘
                   │  len = 5     │
                   │  cap = 5     │
                   └──────────────┘
```

`s`는 `s1`을 가리키는 주소일 뿐이고, 실제 문자열 데이터의 주인은 `s1`입니다.

### 자바의 "참조"와 무엇이 다른가?

자바 개발자에게 "참조"라는 단어는 익숙하지만, 의미가 꽤 다릅니다.

| | 자바의 객체 참조 | 러스트의 참조자 `&T` |
|---|---|---|
| 소유 여부 | 모든 참조가 동등(GC 입장에서 "살려두는" 근거) | **소유하지 않음**. 원본 수명에 영향 없음 |
| null 가능 | 가능 (`NullPointerException`) | **불가능**. 항상 유효한 값을 가리킴이 보장됨 |
| 수정 권한 | 참조만 있으면 누구나 객체 수정 가능 | 기본은 **읽기 전용**, 수정하려면 `&mut` 필요 |
| 동시 접근 제어 | 개발자 책임 (`synchronized`, Lock 등) | **컴파일러가 규칙으로 강제** |
| 대상이 먼저 사라지면? | GC가 대상을 살려둠 → 발생 안 함 | 컴파일 에러로 막음 (댕글링 방지) |

> 💡 `s.len()`처럼 참조자에 바로 메서드를 호출할 수 있는 이유는 러스트가 **자동으로 역참조**해주기 때문입니다.
> 직접 역참조할 때는 `*` 연산자를 씁니다 (뒤의 `*n += 1` 예제 참고).

---

## 4. 불변 참조자: 빌린 것은 기본적으로 고칠 수 없다

```rust
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // ❌ 컴파일 에러
}
```

```
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                         +++
```

변수가 기본적으로 불변이듯, **참조자도 기본적으로 불변**입니다.
컴파일러가 해결책(`&mut`)까지 친절하게 알려주는 것도 눈여겨보세요.

### 자바의 `final`과 비교

```java
final StringBuilder sb = new StringBuilder("hello");
sb.append(", world"); // ✅ 자바에서는 가능 — final은 "재할당"만 막을 뿐
// sb = new StringBuilder(); // ❌ 이것만 막힘
```

자바의 `final`은 **변수가 다른 객체를 가리키지 못하게** 할 뿐, **객체 내용 변경은 막지 못합니다.**
러스트의 `&T`는 **가리키는 대상의 내용 자체를 수정할 수 없게** 합니다.

자바에서 이와 가장 비슷한 건 `Collections.unmodifiableList()` 정도인데,
이건 수정 시 **런타임에** `UnsupportedOperationException`을 던집니다.
러스트는 같은 실수를 **컴파일 단계에서** 잡습니다.

---

## 5. 가변 참조자 `&mut` — 빌려서 고치기

```rust
fn main() {
    let mut s = String::from("hello"); // ① 원본 변수가 mut이어야 함

    change(&mut s);                    // ② 호출부에서 &mut으로 빌려줌

    println!("{s}");                   // hello, world
}

fn change(some_string: &mut String) {  // ③ 시그니처도 &mut
    some_string.push_str(", world");
}
```

세 군데 모두 `mut`이 들어가야 합니다. 귀찮아 보이지만 장점이 큽니다.

> **호출하는 코드만 봐도 "이 함수가 내 값을 바꾼다"는 사실이 드러납니다.**

자바에서는 `process(list)`라는 코드만 보고는 `process`가 `list`를 수정하는지 알 수 없습니다.
메서드 본문을 열어보거나 Javadoc을 믿어야 하죠. 러스트에서는 `process(&list)`와 `process(&mut list)`가 문법적으로 구분됩니다.

### 보너스: 자바에서는 못 하는 것 — 원시 타입을 함수에서 바꾸기

```rust
fn add_one(n: &mut i32) {
    *n += 1; // * 로 역참조해서 원본 값을 수정
}

fn main() {
    let mut count = 0;
    add_one(&mut count);
    println!("{count}"); // 1
}
```

자바는 항상 값에 의한 전달(pass-by-value)이라서, `int`를 메서드 안에서 바꿔도 호출자에게 반영되지 않습니다.
그래서 `AtomicInteger`나 `int[]` 같은 우회책을 쓰곤 하죠. 러스트는 `&mut i32`로 깔끔하게 해결됩니다.

---

## 6. 빌림 규칙 — 이 장의 핵심

> **어느 시점이든, 하나의 값에 대해 다음 둘 중 하나만 가능하다.**
> - **가변 참조자(`&mut`) 딱 1개**
> - **불변 참조자(`&`) 여러 개**

자바의 `ReadWriteLock`을 떠올리면 바로 이해됩니다.

| `ReadWriteLock` | 러스트 빌림 |
|---|---|
| 읽기 락(read lock)은 여러 스레드가 동시에 획득 가능 | `&T`는 동시에 여러 개 가능 |
| 쓰기 락(write lock)은 단 하나, 그동안 읽기 락도 불가 | `&mut T`는 단 하나, 그동안 `&T`도 불가 |
| **런타임에** 락을 획득/해제 | **컴파일 타임에** 검사, 런타임 비용 0 |

### 6-1. 가변 참조자 두 개 → 에러

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ❌

    println!("{r1}, {r2}");
}
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{r1}, {r2}");
  |                -- first borrow later used here
```

`r1`이 `println!`에서 쓰일 때까지 살아 있어야 하는데, 그 사이에 `r2`가 같은 데이터를 또 가변으로 빌렸기 때문입니다.

### 6-2. 불변 참조자와 가변 참조자 혼용 → 에러

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // OK
    let r2 = &s;     // OK — 읽기는 여러 명 가능
    let r3 = &mut s; // ❌ 누군가 읽고 있는데 쓰려고 함

    println!("{r1}, {r2}, and {r3}");
}
```

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

불변 참조를 가진 쪽은 "내가 보는 동안 값이 갑자기 바뀌지 않을 것"을 기대합니다.
반면 읽기만 하는 참조끼리는 서로에게 영향을 줄 수 없으니 여러 개여도 괜찮습니다.

### 6-3. 왜 이렇게까지 하나? — 데이터 레이스 방지

**데이터 레이스**는 다음 세 조건이 동시에 성립할 때 발생합니다.

1. 둘 이상의 포인터가 같은 데이터에 동시에 접근한다.
2. 그중 하나 이상이 데이터를 쓴다.
3. 접근을 동기화하는 장치가 없다.

빌림 규칙은 1번과 2번이 동시에 성립하는 상황 자체를 **컴파일되지 않게** 만듭니다.
자바에서 멀티스레드 버그를 재현하려고 로그를 뒤지던 경험이 있다면, 이게 얼마나 큰 차이인지 느껴질 겁니다.

### 6-4. 자바 개발자라면 한 번쯤 본 에러: `ConcurrentModificationException`

빌림 규칙은 멀티스레드뿐 아니라 **단일 스레드 버그**도 잡아줍니다.

**자바 — 런타임에 터짐**

```java
List<Integer> list = new ArrayList<>(List.of(1, 2, 3));
for (Integer x : list) {
    if (x == 2) {
        list.add(4); // 💥 실행 중 ConcurrentModificationException
    }
}
```

**러스트 — 컴파일 자체가 안 됨**

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {          // v를 불변으로 빌려서 순회 중
        if *x == 2 {
            v.push(4);     // ❌ E0502: 순회 중에 가변으로 빌릴 수 없음
        }
    }
}
```

`for x in &v`가 순회하는 동안 `v`를 불변으로 빌리고 있으므로, 그 안에서 `v.push()`(가변 빌림)를 할 수 없습니다.
`push`로 벡터가 재할당되면 반복자가 해제된 메모리를 가리킬 수 있는데, 러스트는 이를 원천 차단합니다.
**자바가 런타임에 검사(`modCount`)하는 것을, 러스트는 컴파일러가 대신 합니다.**

### 6-5. 스코프를 나누면 여러 번 빌릴 수 있다

"동시에"만 아니면 됩니다. 중괄호로 새 스코프를 만들면 순차적으로 여러 번 가변 대여가 가능합니다.

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
    } // r1이 여기서 사라짐

    let r2 = &mut s; // ✅ 문제없음
    r2.push_str("!");
}
```

---

## 7. 참조자의 수명은 "마지막으로 사용된 곳"까지 (NLL)

최신 러스트에서 참조자의 유효 범위는 **선언된 지점부터 마지막으로 사용된 지점까지**입니다.
블록 끝(`}`)까지가 아닙니다. 이를 **NLL(Non-Lexical Lifetimes)** 이라고 부릅니다.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    // r1, r2는 이 이후로 쓰이지 않음 → 여기서 빌림 종료

    let r3 = &mut s; // ✅ OK — 불변 빌림과 겹치지 않음
    println!("{r3}");
}
```

6-2의 에러 코드와 비교해 보세요. 차이는 **`r1`, `r2`를 `r3` 이후에도 쓰느냐**뿐입니다.

```rust
    let r3 = &mut s;
    println!("{r1}"); // ← 이 한 줄이 있으면 r1의 빌림이 r3와 겹쳐서 ❌
```

> 💡 실무 팁: 빌림 에러가 나면 **"이 참조자를 마지막으로 쓰는 곳이 어디인가?"** 를 먼저 보세요.
> 사용 순서만 바꿔도 해결되는 경우가 많습니다.

---

## 8. 댕글링 참조자 — 자바에는 없는 개념

**댕글링 포인터(dangling pointer)** 는 이미 해제된 메모리를 가리키는 포인터입니다.
C/C++에서는 흔한 버그지만, 자바 개발자에게는 생소할 수 있습니다. GC가 참조가 남아 있는 객체를 절대 해제하지 않기 때문이죠.

러스트에는 GC가 없으므로 이 문제가 실제로 생길 수 있는데, **컴파일러가 막아줍니다.**

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {             // String의 참조를 반환하겠다고 선언
    let s = String::from("hello");   // s는 이 함수가 소유

    &s                               // s의 참조를 반환
} // ← 여기서 s의 소유자 스코프가 끝나 String이 해제됨. 반환된 참조는 허공을 가리킴!
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
...
help: instead, you are more likely to want to return an owned value
  |
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
```

"라이프타임(lifetime)"이라는 개념은 10장에서 다루니 일단 넘어가고, 핵심 메시지만 보면 됩니다.
**"반환 타입이 빌린 값을 담고 있는데, 빌려올 원본이 없다."**

### 자바 개발자의 직관과 비교

```java
// 자바: 아무 문제 없음
String makeGreeting() {
    String s = new String("hello");
    return s; // 참조를 반환해도 GC가 객체를 살려둠
}
```

자바에서는 지역 변수로 만든 객체의 참조를 반환해도 GC가 객체를 살려두니 괜찮습니다.
러스트에서는 함수가 끝나면 소유자 `s`와 함께 값이 해제되므로, **참조가 아니라 값 자체(소유권)를 넘겨야** 합니다.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 소유권이 호출자에게 이동 → 해제되지 않음
}
```

결과적으로 자바 코드와 생김새가 거의 같아집니다. 차이는 "GC가 살려준다" 대신 "소유권이 호출자에게 넘어간다"는 것뿐입니다.

---

## 9. 참조자의 규칙 정리

1. **어느 시점이든 `&mut` 하나 *또는* `&` 여러 개 중 하나만 가질 수 있다.**
2. **참조자는 항상 유효해야 한다.** (null도, 댕글링도 없다)

---

## 10. 함수 시그니처 고르기 — 실전 가이드

자바에서는 매개변수 타입만 고르면 끝이지만, 러스트에서는 **"얼마나 가져갈 것인가"** 도 함께 고릅니다.

| 시그니처 | 의미 | 자바식으로 표현하면 | 언제 쓰나 |
|---|---|---|---|
| `fn f(s: &String)` | 읽기만 할게, 빌려줘 | 읽기 전용 뷰를 받음 | **가장 기본. 일단 이걸로 시작** |
| `fn f(s: &mut String)` | 고칠게, 잠깐 빌려줘 | 인자를 수정하는 메서드 (단, 독점 접근 보장) | 원본을 변경해야 할 때 |
| `fn f(s: String)` | 아예 나한테 넘겨 | 객체를 넘기고 호출자는 더 이상 안 씀 | 값을 저장하거나 변환해서 반환할 때 |

> 💡 문자열 매개변수는 실무에서 `&String`보다 `&str`(문자열 슬라이스)을 더 많이 씁니다. 다음 절(4.3 슬라이스)에서 다룹니다.

### 미리보기: 메서드의 `&self` / `&mut self`

5장 구조체에서 보게 될 메서드도 같은 규칙을 따릅니다.

```rust
struct Account { balance: i64 }

impl Account {
    fn balance(&self) -> i64 { self.balance }            // 자바의 getter 느낌 — 읽기 전용
    fn deposit(&mut self, amount: i64) { self.balance += amount; } // setter/변경 메서드
}
```

자바에서는 getter와 setter의 차이가 관례일 뿐이지만, 러스트에서는 **`&self`냐 `&mut self`냐가 시그니처에 명시되고 컴파일러가 검사**합니다.

---

## 11. 자바 vs 러스트 최종 비교표

| 상황 | 자바 | 러스트 |
|---|---|---|
| 객체를 메서드에 넘김 | 참조 복사, 호출자도 계속 사용 | `&x`로 빌려주면 호출자도 계속 사용 |
| 메서드가 인자를 수정하는지 | 시그니처만으로 알 수 없음 | `&mut`가 호출부와 시그니처에 드러남 |
| 읽기 전용 강제 | `final`은 재할당만 막음, 불변 컬렉션은 런타임 예외 | `&T`는 컴파일 타임에 수정 금지 |
| 순회 중 컬렉션 수정 | 런타임 `ConcurrentModificationException` | 컴파일 에러 (E0502) |
| 멀티스레드 동시 쓰기 | 개발자가 락으로 직접 관리 | 빌림 규칙으로 컴파일 타임에 차단 |
| null 참조 | `NullPointerException` 가능 | 참조자는 null이 될 수 없음 |
| 지역 객체 참조 반환 | GC 덕분에 문제없음 | 컴파일 에러 → 소유권을 반환해야 함 |

---

## 12. 번역본(2판)과 최신 공식 문서의 차이

번역본으로 공부하다 보면 실제 컴파일러 동작과 달라 헷갈릴 수 있는 부분입니다.

| 항목 | 번역본(2판) | 최신 공식 문서 |
|---|---|---|
| 참조자의 유효 범위 | 블록(`}`) 끝까지로 설명 (에러 메시지에 `first borrow ends here`가 `}` 위치로 표시) | **마지막 사용 지점까지** (NLL). 사용이 끝나면 같은 블록 안에서도 새 가변 참조 가능 |
| 에러 예제 | `let r1 = &mut s; let r2 = &mut s;`만 있어도 에러로 설명 | 최신 컴파일러는 이후에 `r1`을 **사용해야** 에러. 그래서 예제에 `println!("{r1}, {r2}")`가 추가됨 |
| 불변 참조 수정 에러 | `cannot borrow immutable borrowed content ...` | `error[E0596]: ... as it is behind a & reference` + `&mut` 제안 |
| 댕글링 에러 메시지 | `'static` 라이프타임 사용을 권함 | `'static`은 흔치 않다고 하고, **소유된 값(`String`) 반환**을 더 권장 |
| 포맷 문자열 | `println!("{}", s1)` | `println!("{s1}")` (변수 직접 삽입, Rust 2021+) |

---

## 다음 단계

- **4.3 슬라이스**: 컬렉션의 "일부"를 빌리는 또 다른 종류의 참조자 (`&str`, `&[T]`)
- **10.3 라이프타임**: 댕글링 예제의 에러 메시지에 나온 `'a`, `'static`의 정체
