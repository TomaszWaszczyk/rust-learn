fn main() {

    let var: u32 = 4;
    increment(var);


    println!("The value of a variable is: {}", var);
}
// the ownership is not transfered, is copied(!!!!!!!!)
fn increment(mut var: u32) -> u32 {
    var += 1;
    var
}
