// The cases with panic! are intended to run with RUST_BACKTRACE=1 to see the 
// full backtrace. Execute this:
//      RUST_BACKTRACE=1 cargo run
//
// You can also set the variable in the environment:
//      export RUST_BACKTRACE=1
// and then restore it:
//      unset RUST_BACKTRACE=1

use std::fs::File;
use std::io::ErrorKind;

fn disaster() {
    let v = vec![4,5,6];
    v[88];
}

fn main() {
    //panic!("Goodbye, cruel world!");      // Uncomment to see it panicking

    //disaster();       // Uncomment to see it panicking

    // Error handling with Result<T,E> enum
    // Result is brought into scope in the prelude
    
    /*let file_result = File::open("myFile.txt");

    let _file = match file_result {
        Ok(the_file) => the_file,
        Err(the_error) => {     // Panic and show the error
            panic!("Problem opening the file: {:?}", the_error);
        }
    };*/

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

    // Shortcuts:

    // Unwrap. This method panics if the file can't be opened
    //let _file3 = File::open("myFile3.txt").unwrap();

    // Expect. This file also panics, but it allows specifying a custom message
    // before the default error message
    let _file4 = File::open("myFile4.txt")
        .expect("I couldn't open the file, sorry. Error:");


}
