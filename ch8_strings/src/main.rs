fn main() {

    // Ways of creating strings:
    let empty_name = String::new();
    let mut name = String::from("Pepe ");
    let another_name = "a_string_slice".to_string();
    let mut number_as_string = 7.to_string();   // Any type implementing Display() can become a string

    // Appending a character with "push()"
    number_as_string.push('5');
    println!("number_as_string {number_as_string}");

    // Appending strings. Different approaches:

    name.push_str(" Martínez ");     // All UTF-8 chars are valid
    println!("Name: {name}");       // We can access name because push_str() does not
                                    // take ownership.

    let second_surname = String::from("García");

    // (+) operator is an alias for:  add(self, s: &str) -> String   
    // method, which uses &str as a parameter and returns a new String

    let mut full_name = name + &second_surname;

    // In this case:
    // &second_surname is actually a &String, but it is coerced into &str.
    // We have lost ownership of "name", because it is borrowed as "self" in the function
    // The contents of name are moved into full_name and then the contents of second_surname
    // are copied.

    println!("Second surname: {second_surname}");       // the second operand is still valid
    println!("Full name: {full_name}");

    // Appending strings using "format" macro
    full_name = format!("Paco Martínez {second_surname}");
    println!("Full formatted name: {full_name}");

    // Iterating over strings with "complex" characters:

    let cyrilic = String::from("йт");
    for c in cyrilic.chars() {      // These chars don't necessarily are 1 byte long
        println!("{c}");
    }

    for b in cyrilic.bytes() {      // A string can always be separated in bytes,
        println!("{b}");            // but multibyte characters will not make sense
    }                               // splitted this way.
}
