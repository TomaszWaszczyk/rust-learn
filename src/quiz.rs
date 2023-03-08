// https://dtolnay.github.io/rust-quiz

trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = || { (return) || true; };
    x().f();

    let x = loop { (break) || true; };
    x.f();

    let x = || { return (|| true); };
    x().f();

    let x = loop { break (|| true); };
    x.f();

    let x = || { return || true; };
    x().f();

    let x = loop { break || true; };
    x.f();
    
    {
        let a1 = [100, 70, 90];
        let a2 = [80, 80, 100];
        let checker = |s: &[u32]| match s {
            [100, ..] => println!("100"),
            _ => println!(";-)"),
        };

        checker(&a1);
        checker(&a2);
    }
}