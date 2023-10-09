fn main() {
    let s1 = String::from("PEPE");
    let s2 = s1;                        // s1 is dropped here. It has "moved" into s2

    //println!("S1 is: {s1}");          // This won't compile: s1 is no longer valid

    // Rust never does deep-copying of complex data by default

    let s3 = s2.clone();

    println!("S2 is {s2} and s3 is {s3}");

    // However it does it in simple data types. They have the Copy trait
    // Nothing that requires allocation can implement Copy.

    let n1 = 5;
    let n2 = n1;
    println!("n1 = {n1} ; n2 = {n2}");


    // Ownership and functions.

    read_string(s3);                 // s3 is moved into the function
    //read_string(s3);               // This wouldn't compile.

    read_number(n2);                    // n2 is copied into the function
    read_number(n2);                    // n2 is still valid in this scope

    let my_new_number : u32 = return_number();  // The function moves an u32 to this scope
    println!("my_new_number = {my_new_number}");

    let mut s4 = take_and_give_back(s2);    // Declared as "mut" only to allow later modifications
    println!("s4 is still valid after being returned: {s4}");


    // References and Mutable References

    borrower(&s4);       // A immutable reference to s4 is passed
    println!("s4 is still valid after passed as reference: {s4}");

    borrower_and_changer(&mut s4);
    println!("s4 is still valid but now it contains: {s4}");

    // Combination of References and Mut. references

    let mut name = String::from("Antonio Alcantara");

    let ref1 = &name;       // We can create as many coexisting immutable references as we want,
    let ref2 = &name;       // because no one will change the value.

    println!("ref1 is ({ref1}) and ref2 is ({ref2})");

    //let ref3 = &mut name; // Won't compile. ref1 is still valid, as it is going to be used below.

    println!("ref1 is ({ref1})");

    let ref3 = &mut name;   // OK. Now, ref1 and ref2 are totally gone.

}


fn read_string(some_string : String) {
    println!("\tReceived a string: {some_string}");
}   // some_string goes out of scope here, it is dropped and is no longer valid.

fn read_number(some_number : i32) {
    println!("\tReceived a number: {some_number}");
}

fn return_number() -> u32 {
    let any_number : u32 = 9;
    return any_number;
}

fn take_and_give_back(the_string : String) -> String {      // the_string comes into this scope
    println!("\tReceived a string: {the_string}, but gonna give it back");
    return the_string;                                      // and goes back to the outer one.
}

fn borrower(any_string: &String) {
    println!("\tI am borrowing this string by reference: {any_string}");
    //any_string.push_str(" Viyuela");          // This won't compile. We don't own any_string.
}

fn borrower_and_changer(any_string: &mut String) {
    println!("\tI am borrowing this string by mutable reference: {any_string}");
    any_string.push_str(" Viyuela");
    println!("\tI have changed the string into: {any_string}.");
}

