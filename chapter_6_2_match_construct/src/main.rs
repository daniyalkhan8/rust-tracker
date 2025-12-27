#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    Florida,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The US State is {state:?}");
            25
        },
    }
}

fn one_plus(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alabama);

    let penny_value = value_in_cents(penny);
    let nickel_value = value_in_cents(nickel);
    let dime_value = value_in_cents(dime);
    let quarter_value = value_in_cents(quarter);

    let five = Some(5);
    let none_val = None;

    let six = one_plus(five);
    let none_val = one_plus(none_val);

    println!("{penny_value}");
    println!("{nickel_value}");
    println!("{dime_value}");
    println!("{quarter_value}");
}
