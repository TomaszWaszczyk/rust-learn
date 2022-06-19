fn test(n: &i32) {
    println!("{}", n);
}


fn main() {
    // let b = Box::new(5);
    let b = Box::new(Box::new(5));
    println!("b = {}", b);

    test(&b);
}
