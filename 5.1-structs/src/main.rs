#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let user1 = User {
        username: String::from("someusername1"),
        email: String::from("some@mail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user: {}", user1.username);

    let mut user2 = User {
        username: String::from("someusername2"),
        email: String::from("some@mail.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.username = String::from("anotherusername2");
    println!("user: {}", user2.username);

    let user3 = build_user(String::from("some@mail.com"), String::from("someusername3"));
    println!("user: {}", user3.username);

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, Color);

    let black = Color(0, 0, 0);
    let origin = Point(123, 0, black);
    println!("{}", origin.0)
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
