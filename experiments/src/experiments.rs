// fn main() {
//     let var: u32 = 4;
// }

// fn increment(mut var: u32) -> u32 {
//     var = var + 1;
//     var
// }
/* ************************************************************** */
// fn bunch_of_numbers() -> Vec<u32> {
//     let mut nums = Vec::new();
//     for i in 0..10 {
//         num.push(i); //(re-)allocation
//     }
//     nums //move
// }

// fn main() {
//     let nums = bunch_of_numbers(); //obtain ownership

//     match nums.last() {
//         Some(&0) => println!("Last number is 0"),
//         Some(n) => println!("Last number is {}", n),
//         None => println!("No number was found"),
//     }
// } //deallocation

/* ************************************************************** */

fn main() {
    let r: &i32;            // ----------+-- 'a
                            //           |
    {                       //           |
        let x: i32 = 5;     // -+--  'b  |
        r = &x;             //  |        |
    }                       // -+        |
                            //           |
    println!("r: {}", r);   //           | dangling reference
}                           // ----------+
