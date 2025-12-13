fn main() {
    let mut str1 = String::from("Hello");
    str1.push_str(", World!");

    let length = calculate_length(&mut str1);

    println!("{str1}, {length}");
}

fn calculate_length(str1: &mut String) -> usize {
    str1.push_str(" - Modified");
    str1.len()
}