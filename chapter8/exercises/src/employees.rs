/*
 Using a hash map and vectors, create a text interface to allow a user to add
 employee names to a department in a company; for example, “Add Sally to
 Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of
 all people in a department or all people in the company by department, sorted
 alphabetically.
*/

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum MenuOption {
    AddNew,
    GetDepartment,
    GetAll,
    Exit,
}

#[derive(Debug)]
struct Employee {
    pub name: String,
    pub department: String,
}

fn prompt(message: &str) -> String {
    println!("{message}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    input.trim().to_string()
}

fn get_option() -> MenuOption {
    loop {
        let input = prompt(
            "\nSelect option from:\n\
            [0]: Add new employee\n\
            [1]: Get all employees in department\n\
            [2]: Get all employees\n\
            [3]: Exit",
        );

        match input.parse::<u32>() {
            Ok(0) => break MenuOption::AddNew,
            Ok(1) => break MenuOption::GetDepartment,
            Ok(2) => break MenuOption::GetAll,
            Ok(3) => break MenuOption::Exit,
            _ => continue,
        }
    }
}

pub fn add_employee(directory: &mut HashMap<String, Vec<String>>) {
    let name = prompt("Enter name:");
    let department = prompt("Enter department");

    let names = directory.entry(department).or_default();
    let position = names.binary_search(&name).unwrap_or_else(|i| i);
    names.insert(position, name);
}

pub fn get_all_employees(directory: &mut HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = directory.keys().collect();
    departments.sort();

    for department in departments {
        print_department(department, &directory[department]);
    }
}

fn print_department(department: &str, names: &[String]) {
    println!("{department}:");
    for name in names {
        println!("- {name}");
    }
}

fn get_department(directory: &mut HashMap<String, Vec<String>>) {
    let department = prompt("Enter department:");

    match directory.get(&department) {
        Some(names) => print_department(&department, names),
        None => println!("No department called {department}"),
    }
}

pub fn exercise_3() {
    println!("Exercise 3: Text interface for employees in departments");

    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let option = get_option();

        match option {
            MenuOption::AddNew => add_employee(&mut directory),
            MenuOption::GetDepartment => get_department(&mut directory),
            MenuOption::GetAll => get_all_employees(&mut directory),
            MenuOption::Exit => break,
        }
    }
}
