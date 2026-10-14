#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn display_points() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };

    println!("Integer point: {integer:?}");
    println!("Float point: {float:?}");
    println!("Mixed point: {mixed:?}");
    println!();
}
