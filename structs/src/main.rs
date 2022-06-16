#[derive(Debug)]
struct Employee {
    name: String,
    id: i64,
}

fn main() {
    let employee = Employee {
        name: "Tomek".to_string(),
        id: 1010,
    };

    println!("My greeting: name: {:#?}", employee);
}
