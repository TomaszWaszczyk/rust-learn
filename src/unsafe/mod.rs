mod m {
    pub struct S {
        p: *const i32,
    }

    impl S {
        pub fn new() -> Self {
            self { p: &42 }
        }

        pub unsafe fn set(&mut self, p: *const i32) {
            self.p = p;
        }

        pub fn dereference(&self) -> i32 {
            unsafe {
                *self.p
            }
        }
    }
}
