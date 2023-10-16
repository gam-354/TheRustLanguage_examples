#![allow(unused)]

// Enums can be used as in C++ to enumerate possible values of a single type:

enum EngineType {
    L4,
    L5,
    L6,
    V6,
    V8,
    V10,
    V12,
}

// But we can also define different enum variants with different simple/complex types:

enum Message {
    Quit,
    Move { x : i32, y : i32 },      // Struct variant
    Write(String),                  // unitary tuple struct
    ChangeColor(i32, i32, i32),     // tuple struct
}

// MATCH

fn read_engine_type(etype : EngineType) -> String {
    return match etype {
        EngineType::L4 => String::from("4 in-line"),
        EngineType::L5 => {                             // Code sections can be executed
            println!("A rare 5 cylinder engine!");
            String::from("5 in-line")                   // This is the returned value
        },
        EngineType::L6 => String::from("6 in-line"),
        _ => String::from("in V"),                      // Wildcard
    };
}

fn plus_one( value : Option<u8> ) -> Option<u8> {
    match value {               // match is an expression itself. It returns a value
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    
    // Declaration of a Message enum instance of Move variant
    let msg = Message::Move {
        x : 20,
        y : -10
    };

    // Usages of Option enum, with None and Some(T) variants:
    // Some and None are already brough to scope in the prelude,
    // so there is no need for the :: disambiguator
    let some_number = Some(5);
    let some_boolean = Some(true);

    // "None" variante does not have type, but belongs to Option<T> enum.
    // absent_number might change its value to a Some<T> value later,
    // so we must define the T although we are now using its None variant.
    let absent_number : Option<u32> = None;

    // Option<T> is not the same type as <T>, so we can not operate
    // instances of different types.
    let _x : u32 = 5;
    let _y : Option<u32> = Some(27);
    // let _sum = _x + _y;              // This won't compile

    // MATCH
    let volvo_v70_petrol_engine = EngineType::L5;
    println!("A Volvo V70 has a {} engine", read_engine_type(volvo_v70_petrol_engine));

    let six = plus_one(Some(5));
    let none = plus_one(None);

    // Match can use a variable to cover the rest of values:

    let dice_value = 5;
    match dice_value {
        6 => println!("repeat!"),
        other => println!("advance {other} steps"),
    };

    // if let is a simplified way to only cover a desired subgroup of cases:

    let number = six;
    if let Some(i) = number {
        println!("Number is {i}");
    }
    
}
