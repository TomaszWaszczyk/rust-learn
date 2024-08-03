use std::{fmt::Debug, process::Output};

trait Producer {
    type Input: Debug + Default;
    type Output: Debug + Default;
    fn produce(&self, input: Self::Input) -> Self::Output;
}

impl Debug for Producer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, {})
    }
}

trait Generic<I: Debug + Default, O: Debug + Default> {
    fn produce(&self, input: I) -> O;
}


fn use_producer(p: impl Producer<Input = u32, Output = String>){}
// fn use_producer(p: impl Producer){}

fn use_generic<I, O>(g: impl Generic<I, O>)
where
I: Debug + Default,
O: Debug + Default
{}

struct A;

impl Producer for A {
    type Input = String;
    type Output = String;

    fn produce(&self, input: String) -> String {
        String::new()
    }
}

impl<I, O> Generic<I, O> for A
where 
I: Debug + Default,
O: Debug + Default
{
    fn produce(&self, input: I) -> O {
        O::default()
    }
}


fn main(){
    let a = A;
    // a.produce(String::new());
    Producer::produce(&a, String::new());
}
