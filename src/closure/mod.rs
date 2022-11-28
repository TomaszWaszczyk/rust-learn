// ⛏️ Take ownership of closure variables with the move keyword

// In Rust, closures are anonymous functions that capture variables from their scope.
// Closures are one of Rust's functional features, along with iterators.

// When defining a closure, the closure borrows any variables from the scope that it utilizes.
// If you want the closure to take ownership, you must use the move keyword. 

// 👉 Closures can capture variables from their scope
// 👉 With the move keyword, we can let the closures take ownership
// 👉 Closures can be stored in variables and passed around

fn my_function(closure: impl Fn()) -> () {
    closure()
}

fn main() {
    let my_data = vec![1, 2, 3, 4, 5];

    let my_closure = || println!("{:#?}", my_data);
    my_function(my_closure);

    let my_move_closure = move || println!("{:#?}", my_data);
    my_function(my_move_closure);
}
