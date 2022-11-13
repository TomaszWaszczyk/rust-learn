mod box_fn;
mod conversions;
mod experiments;
mod handle_errors;
mod iterators;
mod xor;
mod math;

fn main() {
    println!("{:#X}, {:#X}", 0xDEADBEEFu32, 0xC0FEBABEu32);

    // xor::xor();
    // handle_errors::handle_errors();
    // experiments::plus_five(1);
    // iterators::iterators();
    // conversions::make_conversions();
    let sum = math::shorter_summation(100);
    println!("Sum is: {}", sum);
}
