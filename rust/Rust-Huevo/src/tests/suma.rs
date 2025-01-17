// In your source file (e.g., src/my_module.rs)

#[cfg(test)]
mod tests {
    // Import the necessary testing module
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(5 - 3, 2);
    }
}
