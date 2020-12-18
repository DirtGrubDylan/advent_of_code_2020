use std::collections::VecDeque;
#[derive(Debug, PartialEq, Clone, Copy)]
enum NestType {
    Parens,
    Mul,
    None,
}

#[derive(Debug, PartialEq)]
struct Equation {
    data: String,
}

impl Equation {
    fn new(data: &str) -> Equation {
        Equation {
            data: data.to_string(),
        }
    }

    fn get_solution(&self, use_proceedence: bool) -> u128 {
        let mut work: VecDeque<char> = self.data.chars().collect();

        Self::solve(&mut work, use_proceedence, NestType::None)
    }

    fn solve(
        chars_to_process: &mut VecDeque<char>,
        use_precedence: bool,
        nest_type: NestType,
    ) -> u128 {
        let mut result = 0;
        let mut current_operation = '+';

        let mut next_char = chars_to_process.pop_front();

        while next_char.is_some() {
            match next_char {
                Some(x) if x.to_digit(10).is_some() => {
                    let value = next_char.unwrap().to_digit(10).unwrap() as u128;

                    result = Self::perform_operation(result, value, current_operation);
                }
                Some(x) if ((x == '+') || (x == '*')) && !use_precedence => {
                    current_operation = x;
                }
                Some('*') if use_precedence => {
                    let value = Self::solve(chars_to_process, use_precedence, NestType::Mul);

                    result *= value;
                }
                Some('(') => {
                    let value = Self::solve(chars_to_process, use_precedence, NestType::Parens);

                    result = Self::perform_operation(result, value, current_operation);
                }
                Some(')') if nest_type == NestType::Mul => {
                    chars_to_process.push_front(')');

                    break;
                }
                Some(')') if nest_type != NestType::Mul => {
                    break;
                }
                Some(' ') => {}
                Some('+') if use_precedence => {}
                _ => panic!("Weird Char: {:?}", next_char),
            }

            next_char = chars_to_process.pop_front();
        }

        result
    }

    fn perform_operation(first_value: u128, second_value: u128, operation: char) -> u128 {
        match operation {
            '+' => first_value + second_value,
            '*' => first_value * second_value,
            _ => panic!("Weird Char: {}", operation),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Calculator {
    data: Vec<Equation>,
}

impl Calculator {
    pub fn new(data_str: &[String]) -> Calculator {
        Calculator {
            data: data_str.iter().map(|info| Equation::new(info)).collect(),
        }
    }

    pub fn sum_of_solutions(&self, use_precedence: bool) -> u128 {
        self.data
            .iter()
            .map(|equation| equation.get_solution(use_precedence))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 6] = [
        "1 + 2 * 3 + 4 * 5 + 6",
        "1 + (2 * 3) + (4 * (5 + 6))",
        "2 * 3 + (4 * 5)",
        "1 + (8 * 3 + 9 + 3 * 4 * 3) + 3 + 1",
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ];

    #[test]
    fn test_equation_solve_easy() {
        let equation = Equation::new(TEST_DATA[0]);

        let result = equation.get_solution(false);

        let expected = 71;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_solve_hard() {
        let equation = Equation::new(TEST_DATA[5]);

        let result = equation.get_solution(false);

        let expected = 13_632;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_solve_easy_precedence() {
        let equation = Equation::new(TEST_DATA[0]);

        let result = equation.get_solution(true);

        let expected = 231;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_solve_hard_precendence() {
        let equation = Equation::new(TEST_DATA[4]);

        let result = equation.get_solution(true);

        let expected = 669_060;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculator_sum_of_solutions() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let calculator = Calculator::new(&input);

        let result = calculator.sum_of_solutions(false);

        let expected = 26_457;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculator_sum_of_solutions_precedence() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let calculator = Calculator::new(&input);

        let result = calculator.sum_of_solutions(true);

        let expected = 694_173;

        assert_eq!(result, expected);
    }
}
