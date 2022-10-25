use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}

fn main() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);
}