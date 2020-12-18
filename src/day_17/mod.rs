mod pocket_dimension;

use crate::util::file_reader::to_string_vector;
use pocket_dimension::{PocketDimension, PocketDimensions};

pub fn run_day_17() {
    let file_input = to_string_vector("inputs/day_17.txt");

    match file_input {
        Ok(input_lines) => {
            let mut pocket_dimension = PocketDimension::new(&input_lines);
            let mut pocket_dimensions = PocketDimensions::new(&input_lines);

            pocket_dimension.run_cycle_to(6);
            pocket_dimensions.run_cycle_to(6);

            let part_1 = pocket_dimension.number_of_active_cubes();
            let part_2 = pocket_dimensions.number_of_active_cubes();

            println!("Day 17 Part 1: {}", part_1);
            println!("Day 17 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}

