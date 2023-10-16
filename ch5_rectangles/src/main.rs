#![allow(unused)]
#[derive(Debug)]        // This is to have the default Debug trait available for this struct
struct Rectangle {
    width : u32,
    height : u32        // Comma for the last field is optional
}

// Independent function that takes a Rectangle reference and calculates its area
fn area(rect : &Rectangle) -> u32 {
    return rect.width * rect.height;
}

// "Implementation block" for Rectangle struct.
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, rect2 : &Rectangle) -> bool {
        if (self.width > rect2.width) && (self.height > rect2.height) {
            return true;
        } else {
            return false;
        };
    }

    // Associated function. It does not use the struct as a parameter
    fn new_square(side : u32) -> Self {
        return Rectangle {width : side, height : side};
    }
}


fn main() {
    let finca = Rectangle {
        width : 10,
        height : 26,
    };

    // Trying to print a default display version of Rectangle struct, using {:?}
    println!("Finca is {:?}", finca);

    // Another way of displaying an instance (or any expression) is using 'dbg!' macro.
    // It is recommended to pass a reference of what we want to debug
    dbg!(&finca);

    // Testing functions and methods

    let area_func = area(&finca);
    println!("Area of finca from function is {area_func}");

    let area_meth = finca.area();
    println!("Area of finca from method is {area_meth}");

    let huerta = Rectangle {
        width : 2,
        height : 5
    };

    let estadio = Rectangle {
        width : 100,
        height : 60
    };

    println!("Can finca hold a huerta? {}", finca.can_hold(&huerta));
    println!("Can finca hold a estadio? {}", finca.can_hold(&estadio));

    // Associated functions are called using struct name followed by ::
    let square = Rectangle::new_square(5);
    dbg!(&square);

}
