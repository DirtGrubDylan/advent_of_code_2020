mod ship;

use crate::util::file_reader::to_string_vector;
use ship::{Action, Ship};

pub fn run_day_12() {
    let file_input = to_string_vector("inputs/day_12.txt");

    match file_input {
        Ok(input_lines) => {
            let actions: Vec<Action> = input_lines.iter().map(|s| Action::new(s)).collect();

            let mut ship = Ship::new();

            ship.navigate(&actions, false);

            let part_1 = ship.manhattan_distance_moved();

            ship.reset();

            ship.navigate(&actions, true);

            let part_2 = ship.manhattan_distance_moved();

            println!("Day 12 Part 1: {}", part_1);
            println!("Day 12 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
