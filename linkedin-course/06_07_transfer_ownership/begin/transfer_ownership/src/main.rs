fn main() {
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: i32) {
    println!("processing propellant {}...", propellant);
}