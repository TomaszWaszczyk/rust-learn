#[derive(Debug, Clone)]
struct Car {
    license_plate: String,
    color: String,
    brand: String,
    model: String,
    speed: i32,
}

impl Car {
    fn new(license_plate: String, color: String, brand: String ,model: String) -> Car {

        Car {
            license_plate,
            color,
            brand,
            model,
            speed: 0,
        }
    }

    fn accelerate(&mut self, delta_v: i32) {
        self.speed += delta_v;
    }
}

fn main() {
    let mut car = Car::new(
        "PL5405".to_string(),
        "black".to_string(),
        "Fiat".to_string(),
        "Maluch".to_string(),
    );

    car.accelerate(50);
    println!("{:?}", car);
}

