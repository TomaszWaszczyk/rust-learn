mod strings {
    pub fn default() {
        let s: &str;
        s.split(",").map(|x| x.into()).collect::<Vec<String>>().into()
    }
}

fn main() {
    strings::default();
}
