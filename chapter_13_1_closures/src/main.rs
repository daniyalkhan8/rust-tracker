use std::thread;
use std::vec;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let num_red = 0;
        let num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red + 1,
                ShirtColor::Blue => num_blue + 1,
            };
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!("{:?}, {:?}", user_pref_1, giveaway_1);

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!("{:?}, {:?}", user_pref_2, giveaway_2);

    let mut numeric_array = vec![1, 2, 3];

    let only_borrows = || println!("{numeric_array:?}");
    only_borrows();

    let mut borrows_mutably = || numeric_array.push(4);
    borrows_mutably();
    println!("{numeric_array:?}");

    thread::spawn(move || println!("From thread: {numeric_array:?}"))
        .join()
        .unwrap();

    let mut rectangles_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    rectangles_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{rectangles_list:#?}, sorted in {num_sort_operations} operations");
}
