pub fn demo(x: impl Iterator<Item = i32>) -> i32 {
    x.sum()
}

fn main() {
    let it = 0..10;
    assert_eq!(demo(it), 45); // static dispatch
    
    let mut it = 0..10;
    let it: &mut dyn Iterator<Item = i32> = &mut it;
    assert_eq!(demo(it), 45); // dynamic dispatch
}
