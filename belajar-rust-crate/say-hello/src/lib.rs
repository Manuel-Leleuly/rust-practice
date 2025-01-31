#[cfg(feature = "hello")]
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(feature = "hello")]
pub fn say_hello_to_everyone() -> String {
    String::from("Hello, everyone!")
}

#[cfg(feature = "bye")]
pub fn say_goodbye(name: &str) -> String {
    format!("Goodbyte, {}", name)
}

#[cfg(feature = "bye")]
pub fn say_goodbye_to_everyone() -> String {
    String::from("Goodbye, everyone!")
}
