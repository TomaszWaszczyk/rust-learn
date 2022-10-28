use std::io;
use std::collection::{BTreeMap};

fn take_number() -> isize {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("error");
    number.trim().parse::<isize>().unwrap()
}

fn take_string() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut number).expect("error");
    string.trim().parse::<isize>().unwrap()
}

fn main(){
    loop {
        let mut dictionary: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for _ in 0..n {
        let string = read_string();
        let substrings = s.split_whitespace();
    
        let mut person = "";
        for (i, substrings) in substrings.enumerate(){
        if i == 0 {
            person = substring
        } else {
            dictionary.entry(substring).or_insert(vec![]).push(person);
        }

        }
    }
    
    for (key, mut value) in dictionary {
        println!("{} ", key);
        value.sort();
        for person in value {
            println!("{}", person);
        }
        println!(); //empty line
    }
    println!(); //empty line
    }
}
