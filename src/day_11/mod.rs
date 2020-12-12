mod seating_system;

use crate::util::file_reader::to_string_vector;
use seating_system::SeatingSystem;

pub fn run_day_11() {
    let file_input = to_string_vector("inputs/day_11.txt");

    match file_input {
        Ok(input_lines) => {
            let mut seating_system = SeatingSystem::new(&input_lines);

            seating_system.simulate_until_stable();

            let part_1 = seating_system.number_of_occupied_seats();

            seating_system.reset();

            seating_system.simulate_until_stable_with_los();

            let part_2 = seating_system.number_of_occupied_seats();

            println!("Day 11 Part 1: {}", part_1);
            println!("Day 11 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
