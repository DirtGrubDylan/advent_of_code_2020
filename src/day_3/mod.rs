mod map;

use crate::file_reader::to_string_vector;
use map::TreeMap;

pub fn run_day_3() {
    let file_input = to_string_vector("inputs/day_3.txt");

    match file_input {
        Ok(input_lines) => {
            let map = TreeMap::new(&input_lines);

            let part_1 = map.number_of_trees_in_line(3, 1);

            println!("Day 3 Part 1: {:?}", part_1);

            let part_2 = map.number_of_trees_in_line(1, 1)
                * map.number_of_trees_in_line(3, 1)
                * map.number_of_trees_in_line(5, 1)
                * map.number_of_trees_in_line(7, 1)
                * map.number_of_trees_in_line(1, 2);

            println!("Day 3 Part 2: {:?}", part_2);
        }
        Err(err) => println!("Error parsing file: {:?}", err),
    };
}
