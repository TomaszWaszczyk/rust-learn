

fn main() {

}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result= greeting("Tomek");
        assert!(result.contains("Tomek"));
    }
}
