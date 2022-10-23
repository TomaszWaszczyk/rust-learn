mod handle_errors;
mod experiments;

fn main() {
    handle_errors::handle_errors();
    experiments::plus_five(1);
}
