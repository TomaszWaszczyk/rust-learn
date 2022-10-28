

// If I have impl X for Y and impl<T: X> X for &T, 
//why would I still need impl X for &Y? And why do I only see this when 
//using it in a tuple? What am I missing?

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

