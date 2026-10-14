use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            _ => {
                panic!("Problem opening file: {error:?}");
            }
        },
    };

    let mut file_contents = String::new();
    greeting_file.read_to_string(&mut file_contents);
    println!("{file_contents}");

    let username = read_username_from_file().unwrap();

    println!("{username}");

}

fn read_username_from_file() -> Result<String, io::Error> {
    // This whole function could be written as fs::read_to_string("hello.txt"),
    // but this implementation show how to use the ? operator for error 
    // propagation. This only works in a function that returns `Result``, 
    // `Option`, or another type that implements `FromResidual`.
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

