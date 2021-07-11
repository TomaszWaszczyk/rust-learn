fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}