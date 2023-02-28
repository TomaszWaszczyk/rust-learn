// https://www.possiblerust.com/pattern/non-generic-inner-functions
// https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

// 🐖 Separate your algorithm from your data structure with the visitor pattern in Rust
// The visitor pattern is a behavioral design pattern that lets us separate our algorithms from our data.
// It enables us to add new behaviors to existing code without altering the current logic.
// Serde, the most popular Rust crate, uses the visitor pattern for its deserializer to visit all elements of the structure to deserialize.
// It can be a great pattern if you want to operate on all elements in a complex data structure while keeping the code clean and maintainable.

trait Visitor {
    type Value;

    fn visit_string(&self, _value: String) -> Self::Value {
        unimplemented!()
    }

    fn visit_bool(&self, _value: bool) -> Self::Value {
        unimplemented!()
    }
}

struct StringVisitor;
impl Visitor for StringVisitor {
    type Value = String;

    fn visit_string(&self, value: String) -> Self::Value {
        value
    }
}

struct BoolVisitor;
impl Visitor for BoolVisitor {
    type Value = bool;
    fn visit_bool(&self, value: bool) -> Self::Value {
        value
    }
}

trait Deserializer<V: Visitor> {
    fn deserialize_string(&self, value: &str) -> V::Value;
    fn deserialize_bool(&self, value: bool) -> V::Value;
}

struct StringDeserializer<V: Visitor>(V);
impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn deserialize_string(&self, value: &str) -> V::Value {
        self.0.visit_string(value.into())
    }
    fn deserialize_bool(&self, _value: bool) -> V::Value {
        unimplemented!()
    }
}

struct BoolDeserializer<V: Visitor>(V);

impl<V: Visitor> Deserializer<V> for BoolDeserializer<V> {
    fn deserialize_string(&self, _value: &str) -> V::Value {
        unimplemented!()
    }
    fn deserialize_bool(&self, value: bool) -> V::Value {
        self.0.visit_bool(value)
    }
}

fn main(){
    let string_deserializer = StringDeserializer(StringVisitor);
    assert_eq!(string_deserializer.deserialize_string("Example of string"), "Example of string");
    
    let bool_deserializer = BoolDeserializer(BoolVisitor);
    assert_eq!(bool_deserializer.deserialize_bool(true), true)
}
