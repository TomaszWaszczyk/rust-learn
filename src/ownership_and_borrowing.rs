// https://d3m3vilurr.gitbooks.io/the-unsafe-rust-programming-language/content/uninitialized.html
// https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/

/*
use std::println;

pub fn take(v: Vec<i32>){
   println!("We took v: {}", v[10]);
}

pub fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

pub fn borrow1(v: &Vec<i32>){
    println!("{}", (*v)[10] + (*v)[12]);
}

pub fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]);
}
*/


// i need to to be aware of ownership - coping or moving ownership
#[derive(Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

fn sum(point: &mut Point) -> isize {
    point.x *= 2;
    point.x + point.y
}

fn square(n: isize) -> isize {
    n * n
}

fn main() {
    let a: isize = 10;
    let b: isize = a;
    let c: isize = square(a);

    println!("The a variable is equal to {}", a);
}
