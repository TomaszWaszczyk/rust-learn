fn main() {
    let a = 100;
    let _b_ref = work(&a);
}

fn work(a_ref: &i32) -> &'static i32 {
    let b = *a_ref + 900;

    Box::leak(Box::new(b))
}
