use std::fmt::Display;

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> &String {
        &self.author
    }
}

#[derive(Debug)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> &String {
        &self.username
    }
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> &String;

    fn notify(&self) -> String {
        format!("Read More From {}", self.summarize_author())
    }
}

fn notify(item: &impl Summary) {
    println!("New Notification: {}", item.summarize());
}

fn re_notify<T>(item: &T)
where
    T: Summary
{
    println!("Re-notifying: {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T> Pair<T>
where
    T: Display + PartialOrd
{
    fn find_largest(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let post = returns_summarizable();

    notify(&post);
    re_notify(&post);

    let pair = Pair::new(65.5, 44.5);
    pair.find_largest();
}
