fn main() {
    let a = some_complex_function()?;

    println!("{:?}", a);
}

fn some_complex_function<T>() -> Option<T> {
    Some(())
}
