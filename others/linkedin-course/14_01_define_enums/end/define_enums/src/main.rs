#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);
}