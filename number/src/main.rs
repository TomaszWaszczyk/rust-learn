use std::collections::HashMap;
// fn main() {
//     let number = 100;

//     if number % 9 == 0 {
//         println!("number is divisible by 9");
//     } else if number % 10 == 0 {
//         println!("number is divisible by 10");
//     } if number % 5 == 0 {
//         println!("number is divisible by 5");
//     } else {
//         println!("number is divisible by 9, 10 or 5");
//     }
// }

// fn main() {
//     let mut i:i32 = 1;
//     let result = loop {
//         i += 1;
//         if i == 11 {
//             break i;
//         }
//     };
//     println!("The result is {}", result);
// }

// fn main() {
//     let s = String::from("hello world");
//     let w = &s[0..5];
//     println!("{}", w);
// }

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}