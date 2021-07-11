use std::fs;

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt");
    println!("contents is: {:?}", contents);
}
