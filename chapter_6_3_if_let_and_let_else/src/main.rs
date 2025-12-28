# [derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

impl UsState {
    fn existed_in(&self, year: i32) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::California => year >= 1900
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_quarter_state(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old state."))
    } else {
        Some(format!("{state:?} is fairly new."))
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);

    let text = describe_quarter_state(coin);
    println!("{text:?}");

    let mut number = Some(5);
    if let Some(num) = &mut number {
        *num += 1;
    }

    assert_eq!(number, Some(6));
}
