fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut v = vec![2, 3, 1, 4, 2, 5];
    let iterator = v.iter_mut();

    print_type_of(&iterator);

    for j in iterator {
        *j += 1;
        // println!("{}", j);
    }

    println!("{:?}", &mut v);
}


