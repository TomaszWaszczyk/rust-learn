fn main() {
    let a = true;
    let b = false;
    println!("a is {} and b is {}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let c = (a ^ b) && panic!();
    println!("c is {}", c);
}
