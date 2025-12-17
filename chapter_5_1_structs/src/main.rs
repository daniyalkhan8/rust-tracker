// Normal Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

// Unit like Struct
struct AlwaysEqual;

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let email = String::from("abc@abc.com");
    let username = String::from("daniyal");

    let user_1: User = build_user(username, email);

    let user2 = User {
        email: String::from("another.email@email.com"),
        ..user_1
    };

    let point = Point(0, 0, 0);
    let color = Color(0, 0, 0);

    let Point(x, y, z) = point;
}
