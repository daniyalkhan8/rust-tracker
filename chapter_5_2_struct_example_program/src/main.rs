#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 30
    };

    let area = calculate_area(&rect);

    println!("{area}");

    dbg!(&rect);
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}