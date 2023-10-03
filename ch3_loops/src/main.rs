fn main() {

    loop {  // Infinite loop, unless stopped by "break" expression
        println!("Hello, world!");
        break;      // We are actually stopping the loop after 1 iteration
    }

    // Returning values from loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {result}");

    // Loop labels

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    // Iterating through collections using "for"

    let a = [10,20,30];

    for element in a {
        println!("The value is {element}");
    }


    // Countdown with "for" using reversed Range 

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("MATOSE");
}
