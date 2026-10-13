use crate::garden::vegetables::Asparagus; // Include asparagus struct from src/garden/vegetables.rs

pub mod garden; // Include code found in src/garden.rs

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}
