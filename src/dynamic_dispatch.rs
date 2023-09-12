struct Circle {
    radius: f64
}

struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn main() {
    // trait objects must include the `dyn` keyword
    let shapes: [&dyn Shape; 3] = [&Circle{radius: 1.3}, &Square{side:2.0}, &Circle{radius: 2.0}];

    // &&dyn Shape
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}
