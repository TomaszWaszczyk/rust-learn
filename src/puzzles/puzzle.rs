// https://stackoverflow.com/questions/48015600/cannot-use-operator-for-functions-that-return-result-error

fn main() {
    let a = some_complex_function()?;

    println!("{:?}", a);
}

fn some_complex_function<T, E>() -> Result<T, E> {
    // Ok(Ok("complex"))
}
