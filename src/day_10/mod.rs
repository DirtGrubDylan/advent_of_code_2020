mod adapter;

use crate::util::file_reader::to_string_vector;
use adapter::Device;

pub fn run_day_10() {
    let file_input = to_string_vector("inputs/day_10.txt");

    match file_input {
        Ok(input_lines) => {
            let device = Device::new(&input_lines, 3);

            let device_joltage_differences = device.get_joltage_differences_using_all_adapters();

            let part_1 = device_joltage_differences.get(&1).unwrap_or(&0)
                * device_joltage_differences.get(&3).unwrap_or(&0);

            let part_2 = device.number_of_unique_adapter_combinations();

            println!("Day 10 Part 1: {:?}", part_1);
            println!("Day 10 Part 2: {:?}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
