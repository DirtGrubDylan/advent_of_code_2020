mod ticket;

use crate::util::file_reader::to_string_vector;
use ticket::TicketScanner;

pub fn run_day_16() {
    let file_input = to_string_vector("inputs/day_16.txt");

    match file_input {
        Ok(input_lines) => {
            let ticket_scanner = TicketScanner::new(&input_lines);

            let part_1 = ticket_scanner.get_error_rate();
            let part_2 = run_part_2(&ticket_scanner);

            println!("Day 16 Part 1: {}", part_1);
            println!("Day 16 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}

fn run_part_2(ticket_scanner: &TicketScanner) -> usize {
    let mut result = 1;

    let ticket_fields = ticket_scanner.get_your_ticket_fields();

    for (field_name, field_value) in ticket_fields.iter() {
        if field_name.starts_with("departure") {
            result *= *field_value;
        }
    }

    result
}
