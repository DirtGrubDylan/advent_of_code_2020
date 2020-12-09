mod password;

use crate::util::file_reader::to_string_vector;

use password::PasswordDebugLine;

pub fn run_day_2() {
    let file_input = to_string_vector("inputs/day_2.txt");

    match file_input {
        Ok(file_lines) => {
            let debug_lines: Vec<PasswordDebugLine> = file_lines
                .iter()
                .map(|line| PasswordDebugLine::new(line))
                .collect();

            let number_of_old_valid_passwords = debug_lines
                .iter()
                .filter(|debug_line| debug_line.old_password_is_valid())
                .count();

            let number_of_valid_passwords = debug_lines
                .iter()
                .filter(|debug_line| debug_line.password_is_valid())
                .count();

            println!("Day 2 Part 1: {}", number_of_old_valid_passwords);
            println!("Day 2 Part 2: {}", number_of_valid_passwords);
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}
