fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("You can't use localhost as host!");
    } else {
        println!("Starting application on {}:{}", host, port);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        println!("Hello test");
    }

    // ==============================

    #[test]
    fn test_add() {
        let result = add(1, 2);
        assert_eq!(result, 3, "1 + 2 should be 3");
    }

    // ===============================

    #[test]
    #[should_panic] // this test succeed if it throws panic
    fn test_start_application() {
        start_application("localhost", 8080);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_ignored() {
        println!("This test is ignored");
    }

    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = add(1, 2);
        if result == 3 {
            Ok(())
        } else {
            Err("1 + 2 should be 3".to_string())
        }
    }
}
