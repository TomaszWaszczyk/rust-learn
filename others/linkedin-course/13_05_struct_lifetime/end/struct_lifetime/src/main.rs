struct Shuttle<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let vehicle = Shuttle {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}