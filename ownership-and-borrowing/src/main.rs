fn main() {
    let x = 1; // x owns 1 
    // 1 is on a heap
    let y = x;

    // scope
    {
        let a = 10;
    }
    // ==================================

    let s = String::from("String");
    let y = s;

    println!("{}", s)
}
