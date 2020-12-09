mod xmas;

use xmas::Xmas;
use crate::util::file_reader::to_string_vector;


pub fn run_day_9() {
    let file_input = to_string_vector("inputs/day_9.txt");

    match file_input {
        Ok(input_lines) => {
            let xmas = Xmas::new(&input_lines, 25);

            let part_1 = xmas.find_first_invalid_element();
            let part_2 = xmas.find_encryption_weakness();

            println!("Day 9 Part 1: {:?}", part_1);
            println!("Day 9 Part 2: {:?}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
