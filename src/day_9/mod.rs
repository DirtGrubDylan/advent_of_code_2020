use crate::file_reader::to_string_vector;

pub fn run_day_9() {
    let file_input = to_string_vector("inputs/day_8.txt");

    match file_input {
        Ok(_input_lines) => {
            unimplemented!()
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}
