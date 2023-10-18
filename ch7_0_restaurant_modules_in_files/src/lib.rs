#![allow(unused)]

mod front_of_house;
mod back_of_house;

// Using "use"
use crate::front_of_house::hosting;     // This shortcut is only valid for its scope, 
                                        // and not for inner or outer scopes
// Using "use" with an alias
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