use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flown.insert("Hadfield", 3); // Chris Hadfield
    missions_flown.insert("Hurley", 3); // Doug Hurley
    missions_flown.insert("Barron", 0); // Kayla Barron
    println!("missions_flown is {:?}", missions_flown);
}