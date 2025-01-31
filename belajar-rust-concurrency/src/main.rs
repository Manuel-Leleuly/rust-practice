use std::thread;

mod main_test;

fn main() {
    println!("Hello, world!");

    let current_thread = thread::current();
    println!("{} : Hello, world!", current_thread.name().unwrap());
}
