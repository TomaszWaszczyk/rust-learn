// start point:
// pub fn make_conversions() {
//     let a: i32 = 2;
//     let b: i64 = 3;

//     let sum = add(a as f64, b as f64);
//     println!("Sum: {}", sum);
// }

// fn add(a: f64, b: f64) -> f64 {
//     a + b
// }

// MAKE CONVERSION =>>

pub fn make_conversions() {
    let a: u16 = 2;
    let b: u8 = 3;

    let sum = add(a, b);
    println!("Sum: {}", sum);
}

fn add<TA: Into<f64>, TB: Into<f64>>(a: TA, b: TB) -> f64 {
    a.into() + b.into()
}
