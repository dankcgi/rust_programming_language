fn main() {
    let username = String::from("user1");
    let email = String::from("user1@example.com");
    let user1 = build_user(username, email);
    print_user(user1);

    let black = Colour(0, 0, 0);
    print_colour(black);
    
    let origin = Point(0, 0, 0);
    print_point(origin);

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
} 

fn print_user(user: User) {
    println!("Username: {}", user.username);
    println!("eMail: {}", user.email);
    println!("Sign-in Count: {}", user.sign_in_count);
    println!("Active: {}", user.active);
}

// Tuple Structs
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn print_colour(colour: Colour) {
    println!("Colour - R: {}, G: {}, B: {}", colour.0, colour.1, colour.2);
}

fn print_point(point: Point) {
    println!("Point - X: {}, Y: {}, Z: {}", point.0, point.1, point.2);
}