#![allow(unused)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Using "use"
use crate::front_of_house::hosting;     // This shortcut is only valid for its scope, 
                                        // and not for inner or outer scopes

use crate::front_of_house as foh;

pub fn eat_at_restaurant() {

    // Three ways of calling add_to_waitlist():

    // a) Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // b) Relative path
    front_of_house::hosting::add_to_waitlist();

    // c) Benefit from having hosting in scope thanks to "use" keyword
    hosting::add_to_waitlist();

    // d) Use + alias
    foh::hosting::add_to_waitlist();
}

fn deliver_order() {}      // functions in main crate are public by default ??

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();     // Accessing outer functions with "super" keyword
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,     // Seasonal fruit is private. This field can't be publicly accessed 
    }

    pub enum Appetizer {        // In contrast, making an enum public will make all its variants public
        Cookie,
        Ham
    }

    impl Breakfast {

        // function to return a Breakfast menu in summer
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),     // Take order of the exact toast requested
                seasonal_fruit: String::from("peaches"),    // Decide to use peaches as fruit
            }
        }
    }
}

fn eat_at_restaurant_in_summer() {
    // Order a breakfast in the summer with Rye toast
    // Call summer() function specifying the type of toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // We can change our mind about the toast type:
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please", meal.toast);

    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes
    // with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}