struct Foo {}

fn test(in1: &mut Foo, in2: &mut Foo) {}

fn main() {
    let mut out1 = [Foo {}, Foo {}];
    let mut out2 = (Foo {}, Foo {});

    test(&mut out1[0], &out1[1]);
    test(&mut out2.0, &out2.1);
}
