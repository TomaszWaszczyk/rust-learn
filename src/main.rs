mod box_fn;
mod conversions;
mod experiments;
mod handle_errors;
mod iterators;
mod xor;
mod math;
mod advent_of_code;

fn main() {
    println!("{:#X}, {:#X}", 0xDEADBEEFu32, 0xC0FEBABEu32);

    // xor::xor();
    // handle_errors::handle_errors();
    // experiments::plus_five(1);
    // iterators::iterators();
    // conversions::make_conversions();
    // let sum = math::shorter_summation(100);
    // println!("Sum is: {}", sum);


    // advent of code
    // "cargo run < ./src/advent_of_code/sample_data/year/2022/day/01/input_part1"
    use advent_of_code::year_2022::day_01::{part1, part2, input};
    let input = input();
    println!("P2: {}", part2(&input));
}
