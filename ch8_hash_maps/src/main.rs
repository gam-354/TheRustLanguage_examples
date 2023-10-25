use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();    // HashMaps don't have initializers

    prices.insert(String::from("Ford"), 20000);
    prices.insert(String::from("Bentley"), 80000);

    //prices.insert(23, 1000);      // Once forced to a type of key, it can't be changed

    // Accessing elements with get()

    let price_ford = prices.get("Ford");        // This is an Option<&i32>

    match price_ford {
        Some(&price) => println!("The price of a ford is {price}"),
        None => println!("This brand is not in the catalog"),
    }

    // Iterating over keys and values at the same time

    for (key, value) in &prices {
        println!("{key} is {value} €");
    }

    // Warning: inserting types that don't implement the Copy trait
    // means that the map becomes the owner of those values and
    // they are borrowed-out from this scope (applies to keys and values)

    let new_brand = String::from("DongFeng");
    prices.insert(new_brand, 16000);

    //println!("The new brand is {new_brand}");   // Won't compile 

    // On the other hand, inserting references to the elements
    // means that they have to remain valid as long as the map is valid.

    // Updating/inserting values in maps

    // This line will add ("Volvo", 30000)
    prices.entry(String::from("Volvo")).or_insert(30000);
    // This line will return Ford's value instead of inserting the new value
    prices.entry(String::from("Ford")).or_insert(30000);

    println!("Ford is {}", prices.entry(String::from("Ford")).or_insert(0));

    
}
