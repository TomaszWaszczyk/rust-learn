struct SomeData {
    value: i32,
}

impl SomeData {
    fn mutate_value(&mut self, new_value: i32) {
        self.value = new_value;
    }
}

fn main() {
    let mut data = SomeData{ value: 10 };
    println!("Original value: {}", data.value);

    data.mutate_value(20);
    println!("Mutated value: {}", data.value);
}
