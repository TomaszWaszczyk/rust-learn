mod ownership_and_borrowing;

fn main() {
    let _x = 1; // x owns 1 
    // 1 is on a heap
    // let y = x;

    // scope
    {
        let _a = 10;
    }
    // ==================================

    let s = String::from("String");
    // let y = s; it does not work

    // borrow s variable for y at the moment
    let _y = &s; // borrowing

    println!("{}", s);

    let mut v = Vec::new();

    for i in 1..100{
        v.push(i);
    }

    ownership_and_borrowing::take(v);
}
