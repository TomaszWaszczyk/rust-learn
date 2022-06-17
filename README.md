# Learn-Rust-by-Building-Real-Applications
```
char* a = …;
int64_t i = *reinterpret_cast<uint64_t*>(a);

Is undefined behavior.
But

int64_t i = …
char* a = reinterpret_cast<char*>(&i);

is perfectly fine?
What is reinterpret_cast even for?!?
```
```String::with_capacity(nb_of_bytes)```
```
https://github.com/rust-lang/rustlings

For now, we recommend that you start by reviewing the Rust Book (Chapters 1-11, as well as Chapter 19), with a special focus on these topics:

Primitive Data Types (Chapter 3.2)
Structs (Chapter 5) 
Enums and matching (Chapter 6)
If & match, For & iterators (Chapter 6)
Modules, crates and file layouts (Chapter 7)
Generics, Types and Traits (Chapter 10)
Visibility (Chapter 10)
Testing (Chapter 11)
Macros (Chapter 19.5)
```
https://d3m3vilurr.gitbooks.io/the-unsafe-rust-programming-language/content/uninitialized.html
https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
https://doc.rust-lang.org/std/slice/
https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/
https://github.com/rust-lang/rfcs/issues/3066


https://github.com/TheAlgorithms/Rust

https://github.com/ctjhoa/rust-learning

https://github.com/pretzelhammer/rust-blog

https://github.com/rust-lang/this-week-in-rust

https://asecuritysite.com/rust/rust_ed25519

https://github.com/rust-in-blockchain/awesome-blockchain-rust

https://docs.rs/

https://cheats.rs/

https://github.com/mre/idiomatic-rust

UI: https://github.com/yewstack/yew

https://www.beginrust.com/

https://rustacean-principles.netlify.app/

https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future/

https://rustwasm.github.io/book/game-of-life/hello-world.html

https://rust-lang.github.io/async-book/

structs, vectors, iteration, Result, Option

https://github.com/mre/idiomatic-rust

https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

https://research.exercism.io/experiment_solutions/a2d59fcea8cd42bf9e81937c831debe1

https://github.com/timvisee/advent-of-code-2020

https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/?source=learn

`cargo clippy` + https://github.com/rust-lang/rustfmt

`cargo clean -p <your_package_name>`

https://github.com/watchexec/cargo-watch -> `cargo watch -x 'run'`

https://rust-lang-nursery.github.io/rust-cookbook/

https://github.com/ralfbiedert/cheats.rs

https://serde.rs/

https://linuxhint.com/list_of_linux_syscalls/

https://doc.rust-lang.org/error-index.html#E0207

!!! https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/crates-and-modules.html

https://w3f.github.io/parachain-implementers-guide/

Debug instructions: https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/

https://jrvidal.github.io/explaine.rs/

https://doc.rust-lang.org/rust-by-example/index.html

https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html

https://doc.rust-lang.org/stable/book/ch03-02-data-types.html

https://www.dataschool.io/how-to-contribute-on-github/

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

https://github.com/rust-lang/rustlings

Compiler explorer: https://godbolt.org/z/hecxae

---

https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior

https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types

https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html

# UI in Rust

https://github.com/emilk/egui

# Questions

1. If I have impl X for Y and impl<T: X> X for &T , why would I still need impl X for &Y ? And why do I only see this when using it in a tuple? What am I missing?

```
pub struct OpaqueHolder(str);

impl OpaqueHolder {
    fn from_borrowed(s: &str) -> &Self {
        unsafe { std::mem::transmute(s) }
    }
    
    fn as_str(&self) -> &str {
        &self.0
    }
}

fn make_opaque() -> &'static OpaqueHolder {
    &OpaqueHolder::from_borrowed("OpaqueHolder")
}


trait Encoder {
    fn as_encoded_string(&self) -> String;

    fn encode(&self) -> String {
        self.as_encoded_string()
    }
}

impl Encoder for OpaqueHolder {
    fn as_encoded_string(&self) -> String {
        return self.as_str().to_string()
    }
}

impl<T: Encoder> Encoder for &T {
    fn as_encoded_string(&self) -> String {
        (&**self).as_encoded_string()
    }
}

// THIS SHOULDNT BE NECESSARY< BUT FOR SOME REASON IS... WHY?
impl Encoder for &OpaqueHolder {
    fn as_encoded_string(&self) -> String {
        return self.as_str().to_string()
    }
}


impl<A: Encoder, B: Encoder> Encoder for (A, B) {
    fn as_encoded_string(&self) -> String {
        [
            self.0.as_encoded_string(),
            self.1.as_encoded_string(),
        ].concat()
    }
}


fn main() {
    // this works w/o the specific &OpaqueHolder
    println!("{:}", make_opaque().encode());
    // only shows up on the tuple
    println!("{:}", (make_opaque(), make_opaque()).encode());
}
```
