struct Shuttle {
    name: String
}

impl Shuttle {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {}", msg);
        &self.name
    }
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Endeavour")
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}