
use std::{ mem };

fn main() {
    println!("Hello, world!");

    let _a:u8 = 123;
    
    let _b:u128 = 123;

    let _z:isize=123;

    let size_of = mem::size_of_val(&_z);
    println!("z = {}, {}-bit OS", size_of, size_of * 8);

    let character:char = 'x'; // or character:char
    println!("character = {}", character);

    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);
}
