/*
 * enum Option<T> {
     None,
     Some(T),
 }
*/
/*
 * Option<T>에서 Some(T)의 값을 꺼내려면 Option의 메서드 또는 match 표현식을 사용한다.
 * Some(T): 내부의 T 값을 꺼내 사용할 수 있다.
 * None: 값이 없으므로 별도의 처리가 필요하다.
 * match: Some과 None을 구분해 각 variant에 맞는 코드를 실행한다.
 */
/*
 * 메서드	                동작	                                      None일 때
 * unwrap()	             내부의 T 값을 꺼냄	                         panic 발생
 * expect("메시지")	    T 값을 꺼냄	                                 지정한 메시지로 panic
 * unwrap_or(default)	 T 값을 꺼냄	                              기본값 반환(즉시 평가)
 * unwrap_or_else(f)     T 값을 꺼냄	                              클로저 f를 실행해 결과 반환(지연 평가)
 * unwrap_or_default()	 T 값을 꺼냄	                              T의 기본값 반환(T::defualt())
 * as_ref()	             Option<T> → Option<&T> 변환 (소유권 이동 없음)	None 유지
 * take()	             내부 값을 꺼내고 기존 값을 None으로 변경	         None 반환
 */
fn main() {}
