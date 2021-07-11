fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
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