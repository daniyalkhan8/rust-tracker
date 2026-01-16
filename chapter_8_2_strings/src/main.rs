fn main() {
    let str1 = String::new();
    println!("{str1}");

    let data1 = "Data 1";
    let s = data1.to_string();
    println!("{s}");

    let s = "Data 1".to_string();
    println!("{s}");

    let mut s = String::from("initial contents");
    println!("{s}");

    let s2 = " for display";
    s.push_str(s2);
    println!("{s}");

    let s = String::from("initial contents");
    let s2 = String::from(" for display");

    let s3 = s + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("{answer}");
}
