use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flown.insert("Hadfield", 3); // Chris Hadfield
    missions_flown.insert("Hurley", 3); // Doug Hurley
    missions_flown.insert("Barron", 0); // Kayla Barron
    missions_flown.insert("Barron", 1);
    missions_flown.entry("Stone").or_insert(2);
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1;
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);
}