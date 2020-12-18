mod calculator;

use crate::util::file_reader::to_string_vector;
use calculator::Calculator;

pub fn run_day_18() {
    let file_input = to_string_vector("inputs/day_18.txt");

    match file_input {
        Ok(input_lines) => {
            let calculator = Calculator::new(&input_lines);

            let part_1 = calculator.sum_of_solutions(false);
            let part_2 = calculator.sum_of_solutions(true);

            println!("Day 18 Part 1: {}", part_1);
            println!("Day 18 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
