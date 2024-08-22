// TODO: Lifetimes can be challenging to understand. The 'static lifetime is particularly tricky because it's often misunderstood as being only for global variables, whereas it can apply to any data that lives for the entire duration of the program.

struct StaticRefExample {
    data: &'static str,
}

fn main() {
    let example = StaticRefExample {
        data: "Hello, world!",
    };
    println!("{}", example.data);
}
