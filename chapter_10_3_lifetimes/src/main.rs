use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl <'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self) -> &str {
        println!("{0}", self.part);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where T: Display
{
    println!("{ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let str1 = String::from("Hello");
    {
        let str2 = String::from("World!");

        let longest_str = longest(&str1, &str2);
        println!("{longest_str}");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{i:?}");

    let part = i.announce_and_return_part();
    println!("{part}")
}
