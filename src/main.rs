mod handle_errors;
mod experiments;
mod xor;
mod iterators;


fn main() {
    // print DEADBEEF - https://pl.wikipedia.org/wiki/0xDEADBEEF
    println!("{:#X}", 0xDEADBEEFu32);
    
    // xor::xor();
    // handle_errors::handle_errors();
    // experiments::plus_five(1);
    iterators::iterators();
}
