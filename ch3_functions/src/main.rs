fn main() {
    println!("Hello, world!");

    print_labeled_measurement(-23, 'p');

    let x = five();
    println!("X = {x}");

    // Branches
    let condition = true;
    let y = if condition {1} else {4};
    println!("Y = {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn five() -> i32 {
    5       // Without ';' so that it is an expression, not a statement
}
