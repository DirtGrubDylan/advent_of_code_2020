mod shuttle;

use crate::util::file_reader::to_string_vector;
use shuttle::ShuttleSystemCalculator;

pub fn run_day_13() {
    let file_input = to_string_vector("inputs/day_13.txt");

    match file_input {
        Ok(input_lines) => {
            let shuttle_system_calculator = ShuttleSystemCalculator::new(&input_lines);

            part_1(&shuttle_system_calculator);
            part_2(&shuttle_system_calculator);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}

fn part_1(shuttle_system_calculator: &ShuttleSystemCalculator) {
    let departure_time_in_minutes = shuttle_system_calculator.departure_time_in_minutes;

    let earliest_shuttle = shuttle_system_calculator
        .get_earliest_shuttle_to_airport()
        .unwrap();

    let shuttle_departure_in_minutes = earliest_shuttle
        .next_closest_departure_to(departure_time_in_minutes)
        .unwrap();

    let part_1 = (shuttle_departure_in_minutes - departure_time_in_minutes) * earliest_shuttle.id;

    println!("Day 13 Part 1: {}", part_1);
}

fn part_2(shuttle_system_calculator: &ShuttleSystemCalculator) {
    let part_2 = shuttle_system_calculator
        .get_timestamp_for_subsequent_departures()
        .unwrap();

    println!("Day 13 Part 2: {}", part_2);
}
