mod ticket;

use crate::util::file_reader::to_string_vector;
use ticket::TicketScanner;

pub fn run_day_16() {
    let file_input = to_string_vector("inputs/day_16.txt");

    match file_input {
        Ok(input_lines) => {
            let ticket_scanner = TicketScanner::new(&input_lines);

            let part_1 = ticket_scanner.get_error_rate();

            println!("Day 16 Part 1: {}", part_1);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
