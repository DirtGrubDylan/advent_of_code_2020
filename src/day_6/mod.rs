mod customs;

use crate::util::file_reader::to_string_vector;

use customs::Group;

pub fn run_day_6() {
    let file_input = to_string_vector("inputs/day_6.txt");

    match file_input {
        Ok(input_lines) => {
            let groups = get_groups_for(&input_lines);

            let part_1 = groups
                .iter()
                .fold(0, |acc, group| acc + group.unique_yes_answers().len());

            let part_2 = groups.iter().fold(0, |acc, group| {
                acc + group.answers_that_everyone_said_yes().len()
            });

            println!("Day 6 Part 1: {:?}", part_1);
            println!("Day 6 Part 2: {:?}", part_2);
        }
        Err(error) => println!("Error Parsing File: {:?}", error),
    };
}

pub fn get_groups_for(input_lines: &[String]) -> Vec<Group> {
    input_lines
        .split(|line| line.is_empty())
        .map(|chunk| Group::new(chunk))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file_reader::to_string_vector;

    #[test]
    fn test_get_groups_for() {
        let input = to_string_vector("test_inputs/day_6.txt").unwrap();

        let result = get_groups_for(&input);

        let expected = vec![
            Group::new(&[String::from("abc")]),
            Group::new(&[String::from("a"), String::from("b"), String::from("c")]),
            Group::new(&[String::from("ab"), String::from("ac")]),
            Group::new(&[
                String::from("a"),
                String::from("a"),
                String::from("a"),
                String::from("a"),
            ]),
            Group::new(&[String::from("b")]),
        ];

        assert_eq!(result, expected);
    }
}
