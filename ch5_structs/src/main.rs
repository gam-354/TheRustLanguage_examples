// STRUCTS

struct Car {
    brand : String,
    model : String,
    year : u32,
    new : bool,
}

fn new_used_car (brand : String, model : String, year : u32) -> Car {
    return Car {
        new : false,    // Fields can be initialized at any order and to default values
        brand : brand,  // Fields can be initialized from parameters
        model : model,  // Fields can be initialized from parameters
        year,           // If no assignation, the compiler will look for a parameter with the same name
    };
}

// TUPLE STRUCTS: structs without field names

struct Position (i32, i32);     // Same amount and type of arguments, but different struct types.
struct MonthYear (i32, i32);

fn main() {
    
    let mut estefania_car = Car {
        brand : String::from("Alfa Romeo"),
        model : String::from("Giulietta"),
        year : 2016,
        new : false
    };

    // If declared as mutable, all fields can be updated (using '=' sign)
    estefania_car.year = 2015;

    let alfa_reus_car = new_used_car(String::from("Alfa Romeo"), String::from("Giulietta"), 2011);

    let another_alfa_car = Car {
        year : 1967,
        ..estefania_car     // This is used to fill the rest of fields from the other instance
    };

    println!("another_alfa_car.brand = {}", another_alfa_car.brand);
    println!("another_alfa_car.model = {}", another_alfa_car.model);
    println!("another_alfa_car.year = {}", another_alfa_car.year);
    println!("another_alfa_car.new = {}", another_alfa_car.new);

    // "estefania_car" is no longer usable, as 'brand' and 'model' are String fields,
    // and the copy done in the construction of "another_alfa_car" was actually a move
    // for those fields, so they were moved to "another_alfa_car". The line below won't compile:
    
    //println!("estefania_car.brand = {}", estefania_car.brand);

    // TUPLE STRUCTS: structs without field names
    let _current_position = Position(22,33);
    let manufacturing_date = MonthYear(10,1993);

    println!("\nManufacturing date: month = {} ; year = {}", manufacturing_date.0, manufacturing_date.1);




}
