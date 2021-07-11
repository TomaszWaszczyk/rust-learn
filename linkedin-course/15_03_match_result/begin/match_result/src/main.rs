use std::fs;

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows the ultimate question!");
    println!("contents is: {:?}", contents);
}
