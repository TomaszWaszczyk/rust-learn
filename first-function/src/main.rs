
fn main() {
    println!("Weight on Mars: {}kg", calculate_weight_on_mars(100.0));

    calculate_weight_on_mars(100.0);
    say_hello("Heeeloooooooooooooooooooooo");
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    50.0
}


fn say_hello(string: &str) -> String {
    format!("Hello to{}", string)
}

fn say_hello_two(string: &str) -> String {
    println!("Hello to{}", string); //-> Not return the expresion
    string.to_string()
}
