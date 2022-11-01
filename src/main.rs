mod handle_errors;
mod experiments;
mod xor;
mod iterators;
mod box_fn;
mod conversions;

fn main() {
    println!("{:#X}, {:#X}", 0xDEADBEEFu32, 0xC0FEBABEu32);
    
    // xor::xor();
    // handle_errors::handle_errors();
    // experiments::plus_five(1);
    // iterators::iterators();
    conversions::make_conversions();
}
