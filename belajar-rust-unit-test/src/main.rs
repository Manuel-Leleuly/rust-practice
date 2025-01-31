mod helper;
mod main_test;

use helper::{say_hello, User};

fn main() {
    println!("Hello, world!");

    let user = User {
        name: String::from("Manuel"),
    };

    println!("{:?}", user);
    say_hello();
}
