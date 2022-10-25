fn main() {
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);
}
