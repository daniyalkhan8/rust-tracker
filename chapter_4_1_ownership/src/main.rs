fn main() {
    let mut str1 = String::from("Hello");
    str1.push_str(", world!");

    // Value str1 moved into function calculate_lenght
    let (str2, length) = calculate_lenght(str1);
    println!("{str2}, {length}");
}

fn calculate_lenght(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}