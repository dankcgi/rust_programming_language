struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn display_points() {
    let integer = Point { x: 34, y: 53 };
    println!("integer.x = {}, integer.y = {}", integer.x(), integer.y());

    let float = Point { x: 1.0, y: 4.0 };
    println!(
        "float.x = {}, float.y = {}, float.distance_from_origin = {}",
        float.x(),
        float.y(),
        float.distance_from_origin(),
    );
    println!();
}
