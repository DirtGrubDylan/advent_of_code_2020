use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct PasswordDebugLine {
    password_policy: PasswordPolicy,
    password: String,
}

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    number_of_letter_range: Range<u32>,
    letter: char,
}

impl PasswordDebugLine {
    pub fn new(debug_line: &str) -> PasswordDebugLine {
        let splitted_line: Vec<&str> = debug_line.split(": ").collect();

        PasswordDebugLine {
            password_policy: PasswordPolicy::new(splitted_line[0]),
            password: splitted_line[1].to_string(),
        }
    }

    pub fn old_password_is_valid(&self) -> bool {
        let mut character_count = HashMap::new();

        for character in self.password.chars() {
            *character_count.entry(character).or_insert(0) += 1;
        }

        let count_of_policy_letter = character_count
            .get(&self.password_policy.letter)
            .unwrap_or(&0);

        self.password_policy
            .number_of_letter_range
            .contains(count_of_policy_letter)
    }

    pub fn password_is_valid(&self) -> bool {
        let first_position = self.password_policy.number_of_letter_range.start;
        let second_position = self.password_policy.number_of_letter_range.end - 1;

        let first_index = (first_position - 1) as usize;
        let second_index = (second_position - 1) as usize;

        let first_index_element = self.password.chars().nth(first_index);
        let second_index_element = self.password.chars().nth(second_index);

        let expected = Some(self.password_policy.letter);

        (first_index_element == expected) ^ (second_index_element == expected)
    }
}

impl PasswordPolicy {
    pub fn new(policy_info: &str) -> PasswordPolicy {
        let range_letter_split: Vec<&str> = policy_info.split(' ').collect();

        let range_split: Vec<&str> = range_letter_split[0].split('-').collect();

        let range = Range {
            start: range_split[0].parse::<u32>().unwrap(),
            end: range_split[1].parse::<u32>().unwrap() + 1,
        };

        let letter = range_letter_split[1].parse::<char>().unwrap();
        PasswordPolicy {
            number_of_letter_range: range,
            letter: letter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALID_DEBUG_DATA: &str = "1-3 a: abcde";
    const TEST_INVALID_DEBUG_DATA: &str = "1-3 b: cdefg";
    const TEST_OLD_VALID_DEBUG_DATA: &str = "2-9 c: ccccccccc";

    const TEST_POLICY_DATA: &str = "2-9 c";

    #[test]
    fn test_password_debug_line_new() {
        let result = PasswordDebugLine::new(TEST_OLD_VALID_DEBUG_DATA);

        let expected = PasswordDebugLine {
            password_policy: PasswordPolicy {
                number_of_letter_range: Range { start: 2, end: 10 },
                letter: 'c',
            },
            password: "ccccccccc".to_string(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_password_debug_line_old_password_is_valid() {
        let debug_line = PasswordDebugLine::new(TEST_OLD_VALID_DEBUG_DATA);

        assert!(debug_line.old_password_is_valid());
    }

    #[test]
    fn test_password_debug_line_old_password_is_not_valid() {
        let debug_line = PasswordDebugLine::new(TEST_INVALID_DEBUG_DATA);

        assert!(!debug_line.old_password_is_valid());
    }

    #[test]
    fn test_password_debug_line_password_is_valid() {
        let debug_line = PasswordDebugLine::new(TEST_VALID_DEBUG_DATA);

        assert!(debug_line.password_is_valid());
    }

    #[test]
    fn test_password_debug_line_password_is_not_valid() {
        let debug_line = PasswordDebugLine::new(TEST_INVALID_DEBUG_DATA);

        let debug_line2 = PasswordDebugLine::new(TEST_OLD_VALID_DEBUG_DATA);

        assert!(!debug_line.password_is_valid());
        assert!(!debug_line2.password_is_valid());
    }

    #[test]
    fn test_password_policy_new() {
        let result = PasswordPolicy::new(TEST_POLICY_DATA);

        let expected = PasswordPolicy {
            number_of_letter_range: Range { start: 2, end: 10 },
            letter: 'c',
        };

        assert_eq!(result, expected);
    }
}
