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
