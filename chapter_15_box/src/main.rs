use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}


fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let y = 5;
    let x = Box::new(y);

    assert_eq!(y, *x);
}
