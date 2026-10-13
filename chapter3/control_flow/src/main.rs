fn main() {
    if_else();
    inline_if_else();
    return_from_loop();
    loop_labels();
    while_loops();
    for_loops();
    concise_for_loops();
    concise_countdown();
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn inline_if_else() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop { // loop labels begin with a single quote
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut countdown = 3;

    while countdown != 0 {
        println!("{countdown}");

        countdown -= 1;
    }

    println!("LIFTOFF!!!")
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn concise_for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn concise_countdown() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}