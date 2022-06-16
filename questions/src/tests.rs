
#[cfg(test)] // enable command: "cargo test"
mod tests {
    use super::*;
    use questions::balanced;

    #[test]
    fn test_balanced() {
        assert_eq!(balanced("[]"), true);
        assert_eq!(backtraced("["), false);
    }
}
