#[cfg(test)]
mod tests {

    #[test]
    fn test_say_hello() {
        let result = hello::say_hello("Manuel");
        assert_eq!(result, "Hello Manuel");
    }
}
