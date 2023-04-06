fn main() {
    let mut value: bool = false;
    let ref_value = &value;

    flip(&mut value); // cannot borrow `value` as mutable because it is also borrowed as immutable
    println!("{}", ref_value);
}

fn flip(value: &mut bool) {
    *value = false;
}
