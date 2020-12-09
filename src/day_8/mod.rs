mod handheld;

use handheld::BootCode;
use crate::util::file_reader::to_string_vector;


pub fn run_day_8() {
    let file_input = to_string_vector("inputs/day_8.txt");

    match file_input {
        Ok(input_lines) => {
            let mut boot_code = BootCode::new(&input_lines);

            boot_code.execute();

            let part_1 = boot_code.get_accumulator();

            boot_code.reset();

            boot_code.execute_self_correcting();

            let part_2 = boot_code.get_accumulator();

            println!("Day 8 Part 1: {}", part_1);
            println!("Day 8 Part 2: {}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
