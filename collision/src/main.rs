use std::fmt;
use sp_core::hexdisplay::HexDisplay;
use rand::{RngCore};
use sha2::{Digest, Sha256};

// number of hashes to generate
const NUMBER_OF_HASHES: i32 = 10_000;
// slice of 0, x of the sha256 hash
const HASH_SIZE: usize = 2;
// the hashes which have more occurences than the below number
const LOWER_THRESHOLD_OF_COUNTS: i32 = 3;
// size of the initial strings generated
const SIZE_OF_RANDOM_STRING: usize = 10;


fn get_small_rand_hex() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut hex = vec![0; SIZE_OF_RANDOM_STRING];
    rng.fill_bytes(&mut hex);
    hex
}

fn get_colliding_hash(input: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(&input);
    let hash = hasher.result();
    let mut hash_bytes_small = [0u8; HASH_SIZE];
    hash_bytes_small.copy_from_slice(&hash[..HASH_SIZE]);
    hash_bytes_small.to_ascii_lowercase()
}

fn get_seed_data() -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..NUMBER_OF_HASHES {
        let tx = tx.clone();
        let handle = std::thread::spawn(move || {
            let s = get_small_rand_hex();
            tx.send(s).expect("Could not send over the channel");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..NUMBER_OF_HASHES {
        v.push(rx.recv().unwrap());
    }
    return v;
}

struct StringWithHash {
    string: Vec<u8>,
    hash: Vec<u8>,
}

fn get_colliding_hash_data(input: Vec<Vec<u8>>) -> Vec<StringWithHash> {
    let mut v = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::new();
    for element in input {
        let tx = tx.clone();
        let handle = std::thread::spawn(move || {
            let hash = get_colliding_hash(element.clone());
            tx.send(StringWithHash {
                string: element,
                hash,
            }).expect("Could not send over the channel");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..NUMBER_OF_HASHES {
        v.push(rx.recv().unwrap());
    }
    return v;
}

#[derive(Debug, Clone)]
struct Collision {
    colliding_inputs: Vec<Vec<u8>>,
    hash: Vec<u8>,
    count: i32,
}

impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // iterate over all colliding inputs and print them
        // join self.colliding_inputs by comma
        write!(f, "Colliding inputs: [")?;
        for input in &self.colliding_inputs {
            // remove the last comma
            if input != &self.colliding_inputs[self.colliding_inputs.len() - 1] {
                write!(f, "{},", HexDisplay::from(input))?;
            } else {
                write!(f, "{}", HexDisplay::from(input))?;
            }
        }
        write!(f, "]")?;
        write!(f, "\nColliding Hash: {}\nNumber of collisions: {}", HexDisplay::from(&self.hash), self.count)
    }
}

fn main() {
    // generate NUMBER_OF_HASHES small rand str and collect into vec, concurrently
    println!("Generating seed data...");
    let seed = get_seed_data();
    println!("Generating colliding hashes...");

    // get  hash for each element from different threads
    let colliding_hash = get_colliding_hash_data(seed);
    println!("Generated colliding hashes");


    println!("Counting colliding hashes...");

    // collect colliding hashes into Vec<Collisions>
    let mut all: Vec<Collision> = Vec::new();
    // seed into struct



    // count colliding hashes and store in struct
    for element in colliding_hash {
        let mut found = false;
        for collision in &mut all {
            if collision.hash == element.hash {
                collision.count += 1;
                collision.colliding_inputs.push(element.string.clone());
                found = true;
            }
        }
        if !found {
            all.push(Collision {
                colliding_inputs: vec![element.string],
                hash: element.hash,
                count: 1,
            });
        }
    }
    
   
    println!("Counted colliding hashes");
    println!("Colliding hashes:");

    for collision in all {
        if collision.count > LOWER_THRESHOLD_OF_COUNTS {
            println!("{}", collision);
        }
    }

}
