// 1. Pattern Match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 2. Patterns that bind to values
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        }
    }
}

// 3. Match with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// 4. Exhaustive
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
}
// >>> ERROR!

// 5. _ Placeholder
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

// 6. Concise Control Flow with if let
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("the maximum is configured to be {}", max),
    _ => (),
}

let config_max = Some(3u8);
if let Some(max) = config_max {
    Some(max) => println!("the maximum is configured to be {}", max);
}

