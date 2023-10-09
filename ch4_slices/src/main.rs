
fn first_word(my_string: &String) -> &str {     // &str = string slice
    let bytes = my_string.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &my_string[0..i];       // slice of 's' from index 0 to i exclusive, i.e. [0,i)
        }
    }

    return &my_string[..];      // Equivalent to &s[0..len]
}

// We can switch to use string slices as inputs

fn more_compatible_first_word(my_string: &str) -> &str {
    let bytes = my_string.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &my_string[0..i];       // slice of 's' from index 0 to i exclusive, i.e. [0,i)
        }
    }

    return &my_string[..];      // Equivalent to &s[0..len]   
}


fn main() {
    let mut full_string = String::from("Manolo Escobar");

    let first = first_word(&full_string);

    println!("first word: {first}");    // slices can be printed

    full_string.clear();                // We empty the original string.
                                        // This is actually a mutable borrow of full_string

    //println!("first word: {first}");  // This won't compile, as it counts as a immutable borrow
                                        // coexisting with the previous mutable borrow

    // Using the "more compatible" first_word() function
    // with different variants of strings:

    let another_full_string = String::from("Salma Hayek");
    let literal_string = "Leopoldo Alas Clarin";

    let word = more_compatible_first_word(&another_full_string);
    let word = more_compatible_first_word(&another_full_string[0..3]);
    println!("first word of {another_full_string} is: {word}");

    let word = more_compatible_first_word(&literal_string);
    let word = more_compatible_first_word(&literal_string[3..11]);
    println!("first word of {literal_string} is: {word}");

    // Note: we are redefining "word", not reassigning it 
    // (we would not be allowed to do that because word is not mutable)

    // Slices also work for other collections:

    let array = [1,2,3,4,5];
    let array_slice = &array[2..4];
    println!("first element of array_slice: {}", array_slice[0]);
}
