fn main() {
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}