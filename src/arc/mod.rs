use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create an Arc instance containing a Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4]));

    // Create a clone of the Arc for each thread
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    // Spawn a thread using one clone
    let handle1 = thread::spawn(move || {
        let mut data = data1.lock().unwrap();
        data.push(5);
        println!("Thread 1: {:?}", data);
    });

    // Spawn another thread using the other clone
    let handle2 = thread::spawn(move || {
        let mut data = data2.lock().unwrap();
        data.push(6);
        println!("Thread 2: {:?}", data);
    });

    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final data
    println!("Main thread: {:?}", *data.lock().unwrap());
}
