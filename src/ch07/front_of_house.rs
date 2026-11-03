// 하위 모듈 본문은 src/ch07/front_of_house/hosting.rs에서 읽는다 (7.5.05).
#[path = "front_of_house/hosting.rs"]
pub mod hosting;

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
