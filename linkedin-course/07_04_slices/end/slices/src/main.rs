fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..];
    println!("last_word is {}", last_word);
    
    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);
}