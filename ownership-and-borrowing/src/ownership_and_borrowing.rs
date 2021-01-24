mod ownership_and_borrowing {

    pub fn take(v: Vec<i32>){
        println!("We took v: {}", v[10], v[10]);
    }
}