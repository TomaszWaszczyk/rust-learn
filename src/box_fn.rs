#[allow(dead_code)]
pub fn test(n: &i32) {
    println!("{}", n);
}

#[allow(dead_code)]
pub fn box_fn() {
    // let b = Box::new(5);
    let b = Box::new(Box::new(5));
    println!("b = {}", b);

    test(&b);
}
