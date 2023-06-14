struct Foo(u8);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{}", self.0);  
    }
}

#[allow(unused_variables, clippy::disallowed_names)]
fn main() {
    {
        Foo(0);
    }
    let foo = Foo(1);
    let _ = Foo(2);
    let _foo = Foo(3);
    Box::leak(Box::new(Foo(4)));
}
