mod passport;

use crate::file_reader::to_string_vector;
use passport::Passport;

pub fn run_day_4() {
    let file_input = to_string_vector("inputs/day_4.txt");

    match file_input {
        Ok(input_lines) => {
            let passports = get_passports(&input_lines);

            let part_1 = passports
                .iter()
                .filter(|passport| passport.contains_required_fields())
                .count();

            println!("Day 4 Part 1: {:?}", part_1);

            let part_2 = get_number_of_valid_passports(&passports);

            println!("Day 4 Part 2: {:?}", part_2);
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    };
}

fn get_passports(input_lines: &[String]) -> Vec<Passport> {
    input_lines
        .split(|line| line.is_empty())
        .map(|chunk| chunk.join(" "))
        .map(|passport_info| Passport::new(&passport_info))
        .collect()
}

fn get_number_of_valid_passports(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.contains_required_fields() && passport.contains_valid_info())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    #[test]
    fn test_get_passports() {
        let file_input = to_string_vector("test_inputs/day_4.txt").unwrap();

        let result = get_passports(&file_input);

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_get_number_of_valid_passports() {
        let file_input = to_string_vector("test_inputs/day_4.txt").unwrap();

        let passports = get_passports(&file_input);

        let result = get_number_of_valid_passports(&passports);

        assert_eq!(result, 2);
    }
}
