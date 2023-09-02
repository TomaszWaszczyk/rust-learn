#[allow(dead_code)]
pub fn iterators() {
    let v: Vec<i32> = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]
    .to_vec();

    let left_limes = 10;
    let right_limes = 15;
    let _iter = (left_limes..right_limes).filter(|x| x % 2 == 0);
    println!("{}", v.len());
}

fn main() {
    let v = [1, 2, 3];
    let mut iter = v.into_iter();

    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());
}
