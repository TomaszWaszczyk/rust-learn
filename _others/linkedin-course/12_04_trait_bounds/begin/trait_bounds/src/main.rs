use std::any;

fn print_type<T>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

fn main() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
}