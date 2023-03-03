fn new<T: Default>() -> T {
    T::default()
}

fn new_with_where<T>() -> T
where
    T: Default,
{
    T::default()
}


fn main() {
    assert_eq!(0.0, new());
    assert_eq!(1.0, new_with_where());
}
