extern crate Lotus;
use Lotus as lotus;
use lotus::*;

mod cli;
mod helpers;
use cli::Calculator;

fn main() {
    let calculator = Calculator::new_from_cli().calculate_payments();
    println!("{}", calculator);
}
