//! 라이브러리 크레이트 루트입니다. `main` 함수 없이 공개 기능을 제공합니다.
//! Cargo는 기본 경로인 src/lib.rs를 라이브러리 크레이트의 루트로 자동 인식합니다.
// mod keyword로 module을 정의하고, module 안에는 다른 module과 struct·enum·constant·trait·function을 둘 수 있다.
/*
 * crate
   └── front_of_house
       ├── hosting
       │   ├── add_to_waitlist
       │   └── seat_at_table
       └── serving
           ├── take_order
           ├── serve_order
           └── take_payment
*/
// Rust의 Absolute Path는 Crate Root(crate) 또는 외부 Crate 이름에서 시작하는 전체 경로이고,
// Relative Path는 현재 모듈을 기준으로 self, super 또는 모듈 이름을 사용해 접근하는 상대 경로이며, 각 경로는 ::로 구분
// ch07에 둔 모듈 파일을 path 속성으로 연결한다.
#[path = "ch07/front_of_house.rs"]
mod front_of_house;

// re-exporting: pub use는 item을 scope로 가져오면서 다른 코드도 그 item을 각자의 scope로 가져올 수 있게 한다.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // seasonal_fruit field is private
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;

    hosting::add_to_waitlist();
}

fn deliver_order() {}
// super로 시작하는 relative path: filesystem의 ..처럼 parent module에서 시작한다.
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
    // struct 앞에 pub을 붙여도 field는 여전히 private이다.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // private field가 있는 struct는 instance를 만드는 public associated function(summer)이 필요하다.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
