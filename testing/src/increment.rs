fn main() {
    let mut v = vec![2, 3, 1, 4, 2, 5];
    let i = v.iter_mut();

    for j in i {
        *j += 1;
        // println!("{}", j);
    }

    println!("{:?}", &mut v);
}
