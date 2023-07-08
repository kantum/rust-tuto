mod front_of_house;

pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change your mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not
    // allowd to see or modify the seasonal frunt that comes
    // with the meal.
    // meat.seasonal_fruit = String::from("Banana");
    let order1 = back_of_house::Appetizer::Pringles;
    let order2 = back_of_house::Appetizer::Tuc;

    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}

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
        Pringles,
        Cacahuet,
        Tuc,
    }
}

// use std::fmt;
// use std::io;
// use std::io::Result as IoResult;
//
// fn function1() -> fmt::Result {
// }
//
// fn function2() -> io::Result<()> {
// }

// fn function3() -> io::IoResult<()> {
// }

