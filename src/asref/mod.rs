pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
    {
        s.as_ref().len()
    }

    pub fn foo() {
        strlen("hello world"); // &'static str
        strlen(String::from("Witaj Swiecie")); // String
    }

fn main() {}
