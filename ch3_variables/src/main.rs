// Note: using variable names starting with "_" to avoid
// "unused variable" and "dead code" warnings

fn main() {

    // Adding "mut" to allow changing the value of existing variables
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Using "const" for constants
    // They can only be assigned to the result of simple expressions, 
    // not to the return value of complex functions
    const _THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    // Shadowing
    let x = 5;
    let x = x + 1;  // Overshadowing original use of x

    {
        let x = x * 2;  // Overshadowing only affects local scope
        println!("The value of X within inner scope is: {x}");
    }

    println!("The value of x in its initial scope is: {x}");

    // Changing variable type by redefinition
    let _spaces = "  ";              // String type
    let _spaces = _spaces.len();      // Number type

    // Type annotations for letting code know how to parse values
    // Try to compile this without the u32 type
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Integers
    let _my_short = 57u8;         // It is 57 in a 8-bit unsigned integer
    let _long_number = 1_000;     // Unsigned 1000, with an underscore to ease legibility

    // Characters
    let _c = 'z';                // Assumes char type
    let _z: char = 'Z';          // char type is 4 bytes logn (unicode)
    let _heart_eyed_cat = '😻';
    println!("Look at these characters: {_c}, {_z}, {_heart_eyed_cat}.");
}
