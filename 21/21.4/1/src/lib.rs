// Define this n a crate called 'adder'.
pub fn add(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crat for test-only use. Cannot be use in non-test code.
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
