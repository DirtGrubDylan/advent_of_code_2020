pub mod file_reader;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use std::io::{self, Write};

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: u32) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        3 => day_3::run_day_3(),
        4 => day_4::run_day_4(),
        5 => day_5::run_day_5(),
        6 => day_6::run_day_6(),
        _ => unimplemented!("I haven't done that day yet :("),
    }
}

pub fn get_user_input() -> u32 {
    let mut input_buffer = String::new();

    io::stdout().flush().expect("Could not flush stdout!");

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input!");

    input_buffer
        .trim()
        .parse::<u32>()
        .expect("Failed to parse user_input!")
}

fn main() {
    print_seperator();

    print!("Please choose a day to run (1-25): ");

    let input = get_user_input();

    print_seperator();

    run_day(input);

    print_seperator();
}
