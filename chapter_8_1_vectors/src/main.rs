enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5];

    vec1.push(575);
    vec1.push(4);
    vec1.push(999);
    vec1.push(321);

    let third = &mut vec2[2];
    *third = 373;
    println!("The third element in vec2 is {third}.");

    let third = vec2.get(4);
    match third {
        Some(value) => println!("The third element in vec2 is {value}."),
        None => (),
    }

    if let Some(value) = third {
        println!("The third element in vec2 is {value}.")
    }

    for i in &mut vec1 {
        *i *= 50;
        println!("{i}")
    }

    let row = vec![
        SpreadSheetCell::Float(32.0),
        SpreadSheetCell::Int(34),
        SpreadSheetCell::Text(String::from("blue"))
    ];
}
