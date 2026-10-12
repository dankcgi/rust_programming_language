fn main() {
    print_labelled_measurement(5, 'h');

    let x = plus_one(5);

    println!("The value of x is {x}");
}

fn print_labelled_measurement(value: u32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // No semi-colon, this is a expression, and gets returned
}