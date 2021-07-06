fn main() {
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value; // NOT
    println!("value is {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("value is {:08b}", value);

    value = value << 4; // shift left by 4
    println!("value is {:08b}", value);

    value = value >> 2; // shift left by 2
    println!("value is {:08b}", value);
}