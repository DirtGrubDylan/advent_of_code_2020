mod aviation_regulations;

use crate::util::file_reader::to_string_vector;

use aviation_regulations::LuggageRegulations;

pub fn run_day_7() {
    let file_input = to_string_vector("inputs/day_7.txt");

    match file_input {
        Ok(input_lines) => {
            let luggage_regulations = LuggageRegulations::new(&input_lines);

            let part_1 = luggage_regulations
                .bags_that_can_contain(&"shiny gold")
                .len();
            let part_2 = luggage_regulations.number_of_bags_to_fill(&"shiny gold") - 1;

            println!("Day 7 Part 1: {}", part_1);
            println!("Day 7 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
