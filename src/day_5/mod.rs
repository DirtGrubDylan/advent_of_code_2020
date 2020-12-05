mod plane;

use crate::file_reader::to_string_vector;
use plane::{BoardingPass, Plane};

pub fn run_day_5() {
    let file_input = to_string_vector("inputs/day_5.txt");

    match file_input {
        Ok(input_lines) => {
            let mut plane = Plane::new(128, 8);

            let boarding_passes: Vec<BoardingPass> = input_lines
                .iter()
                .map(|info| BoardingPass::new(&info))
                .collect();

            plane.fill(&boarding_passes);

            let part_1 = boarding_passes
                .iter()
                .map(|boarding_pass| plane.get_seat_for(&boarding_pass))
                .max_by_key(|seat| seat.get_id())
                .expect("No max found?")
                .get_id();

            let part_2 = plane
                .missing_seats()
                .iter()
                .filter(|seat| plane.surrounding_seats_occupied(seat))
                .map(|seat| seat.get_id())
                .nth(0)
                .expect("Missing Available Seat!");

            println!("Day 5 Part 1: {}", part_1);
            println!("Day 5 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
