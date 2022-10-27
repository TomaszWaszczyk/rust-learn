struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    println!("hubble is {}", hubble);
    println!("iss is {}", iss);
}
