enum MyEnum {
    MyValue(i32),
    MyOtherValue(i32),
    NoValue,
}

fn match_multiple_values(my_enum: MyEnum) -> i32 {
    let (MyEnum::MyValue(number) | MyEnum::MyOtherValue(number)) = my_enum else {
        panic!("No value found in enum")
    };
    number
}

fn match_inclusive_range(number: i32) -> bool {
    if let (5..=6) = number {
        return true;
    }
    false
}

fn main() {
    assert_eq!(match_multiple_values(MyEnum::MyValue(2)), 2);
    assert_eq!(match_multiple_values(MyEnum::MyOtherValue(2)), 2);

}
