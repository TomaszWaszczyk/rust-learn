fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter() {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        mean += num as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
