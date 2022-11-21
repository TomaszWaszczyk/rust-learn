// start point:
// pub fn make_conversions() {
//     let a: i32 = 2;
//     let b: i64 = 3;
//
//     let sum = add(a as f64, b as f64);
//     println!("Sum: {}", sum);
// }

// fn add(a: f64, b: f64) -> f64 {
//     a + b
// }

// MAKE CONVERSION =>>

// pub fn make_conversions() {
//     let a: u16 = 2;
//     let b: u8 = 3;

//     let sum = add(a, b);
//     println!("Sum: {}", sum);
// }

// fn add<TA: Into<f64>, TB: Into<f64>>(a: TA, b: TB) -> f64 {
//     a.into() + b.into()
// }

struct MyStruct {
    pub my_data: String,
}

#[derive(Clone)]
struct MyOtherStruct {
    pub my_other_data: String,
}

impl From<MyOtherStruct> for MyStruct {
    fn from(my_other_struct: MyOtherStruct) -> Self {
        MyStruct {
            my_data: my_other_struct.my_other_data,
        }
    }
}

// #[allow(dead_code)]
// https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#dead-code
fn _main() {
    let my_other_struct = MyOtherStruct {
        my_other_data: "some text".to_string(),
    };

    let my_struct: MyStruct = MyStruct::from(my_other_struct.clone());

    let my_second_struct: MyStruct = my_other_struct.into();

    assert_eq!(my_struct.my_data, my_second_struct.my_data);
}
