### 함수 본문

함수 본문은 필요에 따라 표현식으로 종결되는 구문의 나열로 구성됩니다. 지금까지 우리는 종결 표현식이 없는 함수만
다뤘기에, 표현식이 구문의 일부처럼 여겨질지 모르겠습니다. Rust가 표현식에 기반한 언어기 때문에, 이것은 이해하셔야
하는 중요한 차이점 입니다. 다른 언어들은 이와 같은 차이가 없으니, 구문과 표현식이 함수의 본문에 어떤 식으로 차이나게
적용되는지 살펴보도록 하겠습니다.

### 구문과 표현식

사실 우리는 이미 구문과 표현식을 사용했습니다. *구문*은 어떤 명령들의 나열로 값을 반환하지 않는 어떤 동작을 수행
합니다. *표현식*은 결과 값을 산출해냅니다. 다음 몇 개의 예제를 살펴보도록 합시다. `let` 키워드를 통해 변수를
만들고 값을 할당하는 구문을 만듭니다. 항목 3-3의, `let y = 6;`은 구문입니다:


<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">항목 3-3: 하나의 구문을 갖는 `main` 함수를 선언하였다. </span>

함수 정의는 또 하나의 구문입니다; 상기 예제는 자신 그 자체가 구문입니다. 구문은 값을 반환하지 않습니다.
그러니, 여러분은 다음처럼 `let` 구문을 사용해서는 다른 변수에 값을 대입할 수 없습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

여러분이 이 프로그램을 수행하면, 다음과 같은 에러를 보게 될 겁니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
```

`let y = 6` 구문은 반환 값이 없으므로, `x`에 bind 시킬 것이 없습니다. 이것이 다른 언어와의 차이점 입니다. 이들 언어들은 `x = y = 6`와 같은 코드가 `x`와 `y`에 모두 `6`의 값을
대입할 수 있습니다; Rust에서는 허용되지 않습니다. 여러분이 작성하는 Rust 코드의 대부분은 표현식이며
이는 어떤 값을 산출합니다. `5 + 6`과 같은 간단한 수학 연산을 살펴보면, 이는 `11`이란 값을 산출하는
표현식입니다.

표현식은 구문의 부분일 수 있습니다: 항목 3-3은 `let y = 6;`이란 구문을 갖는데, `6`은 `6`이란 값을
산출하는 표현식입니다. 함수를 호출하는 것은 표현식입니다. 매크로를 호출하는 것은 표현식입니다. 예제처럼 새로운
범위를 생성하는데 사용하는 block `{}`은 표현식입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

표현식 부:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

이번 경우에 해당 block은 `4`를 산출합니다. 이 값은 `let` 구문의 일부로 `y`에 bound됩니다.
여러분이 앞서 봐온 것과 다르게 `x + 1` 줄의 마지막이 세미콜론으로 끝나지 않은 점을 주목하세요.
표현식은 종결을 나타내는 세미콜론을 사용하지 않습니다. 만약 세미콜론을 표현식 마지막에 추가하면,
이는 구문으로 변경되고 반환 값이 아니게 됩니다. 이후부터 함수의 반환 값과 표현식을 살펴보실 때
이 점을 유의하세요.

### 반환 값을 갖는 함수

함수는 그들을 호출한 코드에 값을 반환할 수 있습니다. 우리는 반환되는 값을 명명해야 할 필요는 없지만, 그들의
타입은 화살표(`->`) 뒤에 선언해야 합니다. Rust에서 반환 값은 함수 본문의 마지막 표현식의 값과 동일합니다.
`return` 키워드와 값을 써서 함수로부터 일찍 반환할 수 있지만, 대부분의 함수들은 암묵적으로 마지막
표현식을 반환합니다. 값을 반환하는 함수의 예를 보겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

`five` 함수에는 함수 호출, 매크로, 심지어 `let` 구문도 없이 그저 `5`란 숫자 하나가 있습니다.
이는 Rust에서 완벽하게 함수로 허용됩니다. 함수 반환 값의 타입이 `-> i32`로 명시되어 있다는 점
또한 주목하세요. 해당 코드를 수행하면 다음과 같은 결과를 얻게 될 겁니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
The value of x is: 5
```
