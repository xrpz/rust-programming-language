// slice:  collection 안의 연속된 요소들을 참조하게 해 주며, reference의 일종이라 ownership이 없다

fn first_word_size(s: &str) -> usize {
    // as_bytes로 byte 배열을 얻는다.
    let bytes = s.as_bytes();
    // iter와 enumerate로 (index, 요소 reference) tuple을 얻는다.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // iter() -> byte 배열에 대한 iterator 생성
    // enumerate() -> iter의 결과를 감싸서 각 요소를 tuple의 일부로 반환
    // Iterator::enumerate가 현재 반복 횟수와 다음 값을 함께 주는 iterator를 만든다.
    // iterator는 (i, val) 쌍을 내놓는데 i는 현재 반복의 index, val은 원래 iterator가 반환한 값이다.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn find_second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut cnt = 0;
    let mut pre_word_idx = 0;
    // abc def ghi
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if cnt == 1 {
                return &s[pre_word_idx..i];
            }
            cnt += 1;
            pre_word_idx = i + 1;
        }
    }

    if cnt == 1 {
        return &s[pre_word_idx..];
    }
    s
}

// String slice의 range index는 유효한 UTF-8 문자 경계에 있어야 하며,
// multibyte 문자 중간을 자르면 오류로 종료된다.

fn main() {
    let str = String::from("hello world");
    let first_word_size = first_word_size(&str);
    let find_first_word: &str = find_first_word(&str);

    let find_second_word = find_second_word(&str);

    println!("first word size: {first_word_size}");
    println!("find first word: {find_first_word}");
    println!("find second word: {find_second_word}");
}
