mod camera;

use camera::Image;
use crate::util::file_reader::to_string_vector;

pub fn run_day_20() {
    let file_input = to_string_vector("inputs/day_20.txt");

    match file_input {
        Ok(input_lines) => {
            let image = Image::new(&input_lines);

            let part_1 = image.get_corner_tile_ids().iter().fold(1, |acc, id| acc * id);

            println!("Day 20 Part 1: {}", part_1);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}

