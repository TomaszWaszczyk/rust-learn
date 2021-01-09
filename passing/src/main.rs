fn main() {
    println!("Hello, world!");

    // passByValue("Tomek test");
    my_other_function("Tomek test")
}

fn passByValue(parameter: String) {
    println!("{}", parameter);
}

fn my_other_function(parameter: &str) {
    println!("{}", parameter);
}