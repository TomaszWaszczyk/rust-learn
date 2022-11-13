mod math {
    pub fn summation(n: i32) -> i32 {
        (1..n+1).fold(0, |acc, v| acc + v)
    }

    pub fn shorter_summation(n: i32) -> i32 {
        (1..=n).sum()
    }
}
