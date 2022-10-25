fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    propellant
}