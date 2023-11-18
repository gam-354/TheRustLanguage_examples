// The cases with panic! are intended to run with RUST_BACKTRACE=1 to see the 
// full backtrace. Execute this:
//      RUST_BACKTRACE=1 cargo run
//
// You can also set the variable in the environment:
//      export RUST_BACKTRACE=1
// and then restore it:
//      unset RUST_BACKTRACE=1

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn disaster() {
    let v = vec![4,5,6];
    v[88];
}

fn simple_panic() {
    panic!("Goodbye, cruel world!");    
}

// Error handling with Result<T,E> enum
// Result is brought into scope in the prelude
fn error_handling_1() {
    
    let file_result = File::open("myFile.txt");

    let _file = match file_result {
        Ok(the_file) => the_file,
        Err(the_error) => {     // Panic and show the error
            panic!("Problem opening the file: {:?}", the_error);
        }
    };
}

fn error_handling_2() {
    // We can also act accordingly to the error type:

    let file_result2 = File::open("myFile.txt");

    let _file2 = match file_result2 {
        Ok(the_file) => the_file,
        Err(the_error) => match the_error.kind() {
            ErrorKind::NotFound => {
                match File::create("myFile.txt") {
                    Ok(the_file) => {
                        println!("The file was created");
                        the_file
                    }
                    Err(the_error) => panic!("Problem creating the file. Error: {:?}", the_error)
                }
            },
            other_error => panic!("The file exists but there was an error opening it: {:?}", other_error)
        },
    };
}

// Unwrap. This method panics if the file can't be opened
fn test_unwrap() {
    let _file3 = File::open("myFile3.txt").unwrap();
}

// Expect. This file also panics, but it allows specifying a custom message
// before the default error message
fn test_expect() {
    let _file4 = File::open("myFile4.txt")
        .expect("I couldn't open the file, sorry. Error:");
}

// Check error cases and return them as the Err variant of Result
fn error_propagation_long() -> Result<String, io::Error> {
    let username_file_result = File::open("user.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),    // Stop here and return the error value
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),      // Return username, not what read_to_string says along with Ok
        Err(e) => Err(e),    // Rebuild the error and return it
    }       
}

// The '?' shortcuts will make the function return with the corresponding Err
// if the called inner function returns an error
fn error_propagation_short() -> Result<String, io::Error> {
    
    let mut username = String::new();

    File::open("user2.txt")?.read_to_string(&mut username)?;

    Ok(username)    // This is what the function returns in case of success
}

fn main() {
    
    //simple_panic();     // Uncomment to see it panicking

    //disaster();       // Uncomment to see it panicking

        // Error handling:

    //error_handling_1();
    //error_handling_2();

        // Shortcuts:
    
    //test_unwrap();
    //test_expect();

        // Error propagation:

    let mut result: Result<String, io::Error> = error_propagation_long();
    println!("Error propagation long: {:?}", result);  

    result = error_propagation_short();
    println!("Error propagation short: {:?}", result);  

}
