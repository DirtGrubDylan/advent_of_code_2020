use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn to_string_vector(file_name: &str) -> Result<Vec<String>, String> {
    let file = BufReader::new(File::open(file_name).expect("File not found!"));

    Ok(file
        .lines()
        .map(|line| line.expect("The file is bad!"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_vector() {
        let expected = vec![
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        ];

        let result = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        assert_eq!(result, expected);
    }
}
