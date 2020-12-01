use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct ExpenseReport {
    pub expenses: Vec<u64>,
}

impl ExpenseReport {
    pub fn new(expenses: &[String]) -> ExpenseReport {
        ExpenseReport {
            expenses: expenses
                .to_vec()
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
        }
    }

    pub fn multiply_two_values_that_sum_to(&self, sum_value: u64) -> Option<u64> {
        let mut result = None;

        let mut seen_values = HashSet::new();

        for expense in &self.expenses {
            if sum_value < *expense {
                continue;
            }

            let needed_value_to_reach_sum = sum_value - expense;

            if !seen_values.contains(&needed_value_to_reach_sum) {
                seen_values.insert(expense);
            } else {
                result = Some(expense * needed_value_to_reach_sum);

                break;
            }
        }

        result
    }

    pub fn multiply_three_values_that_sum_to(&self, sum_value: u64) -> Option<u64> {
        let mut result = None;

        for expense in &self.expenses {
            let needed_value_to_reach_sum = sum_value - expense;

            if let Some(product) = self.multiply_two_values_that_sum_to(needed_value_to_reach_sum) {
                result = Some(expense * product);

                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let expenses = vec![String::from("2"), String::from("2018")];

        let result = ExpenseReport::new(&expenses);

        let expected = ExpenseReport {
            expenses: vec![2, 2018],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_from_file() {
        let expenses = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let result = ExpenseReport::new(&expenses);

        let expected = ExpenseReport {
            expenses: vec![1721, 979, 366, 299, 675, 1456],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiply_two_values_that_sum_to() {
        let expenses = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let expense_report = ExpenseReport::new(&expenses);

        let result = expense_report.multiply_two_values_that_sum_to(2020);

        let expected = Some(514579);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiply_three_values_that_sum_to() {
        let expenses = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let expense_report = ExpenseReport::new(&expenses);

        let result = expense_report.multiply_three_values_that_sum_to(2020);

        let expected = Some(241861950);

        assert_eq!(result, expected);
    }
}
