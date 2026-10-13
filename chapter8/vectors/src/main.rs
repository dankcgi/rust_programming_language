#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Type annotation tells us what the vector will contain
    let mut annotated_vector: Vec<i32> = Vec::new();

    // Use macro to load with initial values
    let initial_values = vec![1, 2, 3, 4, 5];

    // Add elements with .push method
    annotated_vector.push(5);
    annotated_vector.push(6);
    annotated_vector.push(7);
    annotated_vector.push(8);

    // Read with an index
    let third: &i32 = &initial_values[2];
    println!("The third element of initial_values is {third}");

    // Read with .get method
    let fourth: Option<&i32> = initial_values.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element of initial_values is {fourth}"),
        None => println!("There is no fourth element in initial_values"),
    }

    // Remove and return the last element with .pop method
    let last = annotated_vector.pop();
    if let Some(last) = last {
        println!("Last element of annotated_vector was {last}, but has been popped off");
    }
    

    // Iterate over immutable vector
    for element in &initial_values {
        println!("{element}");
    }

    // Iterate over mutable vector and change elements
    for element in &mut annotated_vector {
        *element += 50;
        println!("{element}");
    }

    // Store multiple types in the same vector by using an enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        println!("{:?}", cell);
    }
}
