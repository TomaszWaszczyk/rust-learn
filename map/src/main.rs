fn main() {
    println!("Hello, world!");

    
    let v: Vec<i32> = vec![1,2,34,5].into_iter().map(|x| x + 1).rev().collect();


    println!("{:?}", v);
    // v.into_iter().map(|x| x + 1).rev().collect();
    println!("{:?}", v)
}
    