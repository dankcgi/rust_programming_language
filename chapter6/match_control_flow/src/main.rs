use rand::prelude::*;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coins = [
        (String::from("Penny"), Coin::Penny),
        (String::from("Nickel"), Coin::Nickel),
        (String::from("Dime"), Coin::Dime),
        (String::from("Quarter"), Coin::Quarter(UsState::Texas)),
    ];

    for coin in coins {
        let name = coin.0;
        let value = value_in_cents(coin.1);
        println!("The value of a {} is {} cents.", name, value);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Value of six is {:#?}", six);
    println!("Value of none is {:#?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // using the value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // ignore the value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // ignore value and do nothing
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Rolled 3: adding fancy hat");
}

fn remove_fancy_hat() {
    println!("Rolled 7: removing fancy hat");
}

fn move_player(num_spaces: u8) {
    let space_spaces = if num_spaces > 1 { "spaces" } else { "space" };

    println!("Moving {num_spaces} {space_spaces}");
}

fn reroll() {
    let new_roll = roll_d10();
    println!("Reroll value is {new_roll}");
}

fn roll_d10() -> u32 {
    rand::rng().random_range(1..=10)
}

