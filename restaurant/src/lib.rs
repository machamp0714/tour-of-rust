fn serve_order() {}

mod front_of_house; // セミコロンを使うことで、モジュールの中身を同じ名前のファイルから読み込むように命令している。
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); これはコンパイル出来ない

    // use crate::front_of_house::hosting; // useキーワードでスコープにパスを持ち込む
    //hosting::add_to_waitlist();

    use self::front_of_house::hosting; // 相対パスを使用する場合
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult; // asキーワードでエイリアスを指定する
