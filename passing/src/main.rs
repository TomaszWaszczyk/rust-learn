use std::{usize};

fn main() {
    println!("Hello, world!");

    // let mut testString: std::string::String = "Test string";
    let mut var: usize = 100;
    let var_ref: &usize = &var;

    //=1=
    //let mut text: string = "Tomek test"; // <=== can I declare string variable like this??
    
    let stringText = String::from("Tomek tests strings in Rust");
    let stringText_ref: &String = &stringText; 

    // borrowing
    passByValue(stringText); // reference??

    // passByValue(text);
    // my_other_function("Tomek test")
}
// type vs memory
fn passByValue(parameter: String) {
    println!("{}", parameter);
}

//is it borowing???
fn my_other_function(parameter: &str) {
    println!("{}", parameter);
}