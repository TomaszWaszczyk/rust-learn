#[derive(Debug)]    
struct Rectangle<T, U> {
    width: T,
    height: U
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };
    println!("rect is {:?}", rect);
}