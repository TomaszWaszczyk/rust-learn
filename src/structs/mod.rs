// Abstract over const values with const generics in Rust.

// Use const generics to have generic arguments that range over constant
//  values instead of types or lifetimes.

// With this feature, you can abstract over arrays of any size. 
// Before, one had to implement a trait manually for each possible length. 

// The array methods in the standard library were limited to 32 for a long time 
// because this feature was missing.

// 👉 Can be used with integral types
// 👉 Allows to abstract over arrays of any length
// 👉 Reduce runtime complexity by using const bounds

mod structs {
    struct MyStruct<const N: usize = 1> {
        my_list: [i32; N],
    }

    impl<const N: usize> MyStruct<N>{
        fn new() -> Self {
            Self { my_list: [1; N] }
        }

        fn get_list(self) -> [i32; N] {
            self.my_list
        }
    }
}

fn main() {
    let my_struct: structs::MyStruct = MyStruct::new();
    assert_eq!(my_struct.get_list(), [1]);

    let my_struct: MyStruct<3> = MyStruct::new();
    assert_eq!(my_struct.get_list(), [1,1,1]);
}