mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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

    fn fix_incorrect_order() {
        cook_order();

        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path.
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::{self, Write, Result as IoResult};
use std::collections::*;
