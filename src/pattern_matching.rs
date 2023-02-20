// 🛡️ Use pattern matching with if let and let else statements
// The if let and let else statements in Rust are super helpful when matching values when we don't want to be exhaustive like in the match operator.
// You can combine these statements with pattern matching, like in the match operator.
// It can be helpful if you, for example, want to match two enum variants when there are many values or match on an inclusive range.

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
    assert_eq!(match_inclusive_range(5), true);
    assert_eq!(match_inclusive_range(6), true);
    assert_eq!(match_inclusive_range(7), false);
}
