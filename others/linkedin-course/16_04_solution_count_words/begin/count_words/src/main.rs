use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    // read file and build vector of individual words
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occurs
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(1) += 1;
    }
    
    // determine the most commonly used word(s)
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }

    // display results
    println!("Top word(s) occurred {} times:", top_count);
    for word in top_words.iter() {
        println!("{}", word);
    }
}