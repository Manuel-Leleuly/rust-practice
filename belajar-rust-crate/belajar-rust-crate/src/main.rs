use mtl_say_hello::{say_goodbye, say_goodbye_to_everyone, say_hello, say_hello_to_everyone};

fn main() {
    hello::say_hello();
    bye::say_bye();

    let response = say_hello("Manuel");
    println!("{}", response);

    let response = say_hello_to_everyone();
    println!("{}", response);

    let response = say_goodbye("Manuel");
    println!("{}", response);

    let response = say_goodbye_to_everyone();
    println!("{}", response);
}

#[test]
fn test_uuid() {
    let id = uuid::Uuid::new_v4().to_string();
    println!("{}", id);
}
