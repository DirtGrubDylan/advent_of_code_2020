mod messages;

use crate::util::file_reader::to_string_vector;
use messages::RulesAndMessages;

pub fn run_day_19() {
    let file_input = to_string_vector("inputs/day_19.txt");

    match file_input {
        Ok(input_lines) => {
            let rules_and_message = RulesAndMessages::new(&input_lines);

            let part_1 = rules_and_message.number_of_valid_messages_for_rule(0);
            let part_2 = rules_and_message.updated_number_of_valid_messages_for_rule0();

            println!("Day 19 Part 1: {}", part_1);
            println!("Day 19 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
