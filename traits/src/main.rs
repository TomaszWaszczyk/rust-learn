// use traits:: {Tweet, Summary};

// fn main() {
//     println!("Hello, world!");

//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }

// here we have a small struct that represents a Person
#![deny(clippy::all)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

// The default trait, as its name indicates, is a trait that exposes 
// a default() constructor on the types that conform to it and allows
// you to create an instance of that type with default values.
// Many built-in types implement this trait already and you can use those
// default values in your code. You can additionally provide default values for
// your custom types. Here I implement the default trait for our custom Person
// struct, providing a default first and last name.
impl<'a> Default for Person<'a> {
    fn default() -> Self {
        Person {
            first_name: "Too",
            last_name: "Meek"
        }
    }
}

fn main() {
    // i can then put this to use by constructing an instance of Person using "default" constructor
    let person = Person::default();

    println!("First name = {:}", person.first_name);
    println!("Last name = {:}", person.last_name);
}
