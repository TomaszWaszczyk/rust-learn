mod handle_errors;
mod experiments;
mod xor;
mod iterators;
mod box_fn;
mod serde;


fn main() {
    println!("{:#X}", 0xDEADBEEFu32);
    
    // xor::xor();
    // handle_errors::handle_errors();
    // experiments::plus_five(1);
    iterators::iterators();
    serde::serde();
}
