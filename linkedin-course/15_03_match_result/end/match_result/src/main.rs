use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {:?}", error)
        }
    };

    println!("contents is: {:?}", contents);
}