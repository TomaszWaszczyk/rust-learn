struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

/* YOUR CODE GOES HERE */

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
