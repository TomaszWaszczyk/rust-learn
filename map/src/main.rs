fn main() {
    println!("Hello, world!");

    
    let v: Vec<i32> = vec![1,2,34,5].into_iter().map(|x| x + 1).rev().collect();


    println!("{:?}", v);
    // v.into_iter().map(|x| x + 1).rev().collect();
    println!("{:?}", v);
    /*
    https://doc.rust-lang.org/std/iter/struct.Map.html
    #![allow(unused)]
    fn main() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();

    assert_eq!(v, [4, 3, 2]);
    } */

    let slice = ['l', 'i', 'n', 'u', 'k', 's'];
    
    for window in slice.windows(2) {
        println!{"[{}, {}]", window[0], window[1]};
    }

}
