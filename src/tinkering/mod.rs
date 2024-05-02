// https://stackoverflow.com/questions/34363984/what-is-vec

fn return_string() {
    let s = "line1\r\nline2\nline3";
    println!("{:?}", s.lines().collect::<Vec<_>>());
}

fn main() {
    return_string();
}
