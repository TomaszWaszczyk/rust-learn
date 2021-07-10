#[derive(Debug)]    
struct Rectangle {
    width: f64,
    height: f64
}

fn main() {
    let rect = Rectangle {
        width: 1.2,
        height: 3.4
    };
    println!("rect is {:?}", rect);
}