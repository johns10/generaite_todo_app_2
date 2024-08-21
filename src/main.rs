fn initial_message() -> String {
    String::from("Starting Todo App")
}

fn main() {
    println!("{}", initial_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_message() {
        assert_eq!(initial_message(), "Starting Todo App");
    }
}
