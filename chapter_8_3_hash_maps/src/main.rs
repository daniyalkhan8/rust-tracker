use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 50);
    scores.insert("Yellow", 234);
    println!("{scores:?}");

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    let blue_team_score = scores.get("Blue").copied().unwrap_or(0);
    println!("{blue_team_score}");

    scores.insert("orange", 35);
    scores.insert("orange", 123132);

    println!("{scores:?}");

    scores.entry("purple").or_insert(123);
    scores.entry("purple").or_insert(572);

    println!("{scores:?}");

    let mut hello_world_hm = HashMap::new();
    let hello_world_str = "Hello world wonderful world";

    for word in hello_world_str.split_whitespace() {
        let count = hello_world_hm.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{hello_world_hm:?}");
}
