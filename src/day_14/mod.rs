mod docking_data;

use crate::util::file_reader::to_string_vector;
use docking_data::BitmaskSystem;

pub fn run_day_14() {
    let file_input = to_string_vector("inputs/day_14.txt");

    match file_input {
        Ok(input_lines) => {
            let bitmask_system = BitmaskSystem::new_v1(&input_lines);

            let part_1 = bitmask_system.sum_of_memory_values_with_mask();

            println!("Day 14 Part 1: {}", part_1);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
