mod ownership_and_borrowing;

fn main() {

    // let _x = 1; // x owns "1" (1 is literal so is stored on a stack)
    // // each piece of data can have one owner at the rime
    // // 1 is on a heap
    // // let y = x;

    // // scope
    // {
    //     let _a = 10;
    //     {
    //         let s = String::from("String");
    //         // let y =  s;
    //         let y = &s; // borrowing, y is a reference to s

    //         println!("{}", y);
    //     }
    // }
    // // ==================================

    // let s = String::from("String");
    // // let y = s; it does not work

    // // borrow s variable for y at the moment
    // let _y = &s; // borrowing

    // println!("{}", s);

    // let mut v = Vec::new();

    // for i in 1..100{
    //     v.push(i);
    // }

    // ownership_and_borrowing::take(v);

    let first_name = "Tomek".to_string();

    say_first_name(first_name);
    // say_first_name(first_name);// compiler complains
}

fn say_first_name(_first_name: String) {
    println!("{}", _first_name)
}
