fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        // println!("inner_planet is {}", inner_planet); // the line raises an error
    }
    println!("outer_planet is {}", outer_planet);
}
