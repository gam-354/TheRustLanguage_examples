fn main() {
    
    let _victor: Vec<i32> = Vec::new();  // this creates an empty, immutable vector of i32's

    let mut vactor: Vec<i32> = Vec::new();  // Adding elements after creation
    vactor.push(1);
    vactor.push(99);

    let mut vector = vec!['a', 'b', 'c', 'd'];   // vec! macro infers the data type

    // Accessing elements

    let fourth: &char = &vector[3];             // [] operator returns an element or crashes
    println!("Fourth element is {fourth}");
    
    // Accessing elements with Option<T>

    let first: Option<&char> = vector.get(0);
    match first {
        Some(element) => println!("First element is {element}"),
        None => println!("Not found"),
    }

    // Borrowing rules
    // We will modify "vector", therefore "fourth" reference will be dropped,
    // as it was an immutable borrow.

    vector.push('e');   // This is a mutable borrow

    // This won't compile, as it would be an immutable borrow after a mutable one:
    //println!("Look at my fourth element: {fourth}");

    // Iterating

    let voctor : Vec<u32> = vec![20,50,1000];

    for number in &voctor {
        println!("voctor element: {number}");   // number is an immutable reference every loop
    }

    // Iterating and modifying

    let mut vuctor : Vec<u8> = vec![8,1,2,3,4,5];   // Declaring as mutable

    for number in &mut vuctor {     // Note the need for &mut also here
        *number -= 1;               // Note the * used to dereference
    }
    
}
