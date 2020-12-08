use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Operation {
    Accumulate,
    Jump,
    Noop,
}

impl Operation {
    fn new(operation: &str) -> Operation {
        match operation {
            "acc" => Operation::Accumulate,
            "jmp" => Operation::Jump,
            "nop" => Operation::Noop,
            _ => panic!("No Operation For: {}", operation),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn new(instruction_info: &str) -> Instruction {
        let (operation_info, argument_info) = instruction_info.split_at(3);

        let operation = Operation::new(operation_info);
        let argument = Self::get_argument_value(argument_info.trim_start());

        Instruction {
            operation: operation,
            argument: argument,
        }
    }

    fn get_argument_value(argument_info: &str) -> i32 {
        let value_parse_err = format!("Can't Parse Argument Info: {}", argument_info);

        let (sign, value_str) = argument_info.split_at(1);

        let mut value = value_str.parse().expect(&value_parse_err);

        match sign {
            "+" => value *= 1,
            "-" => value *= -1,
            _ => panic!(value_parse_err),
        }

        value
    }

    fn change_operation(&mut self, operation: Operation) {
        self.operation = operation;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Initialized,
    Working,
    Finished,
}

#[derive(Debug, PartialEq)]
pub struct BootCode {
    instructions: Vec<Instruction>,
    current_instruction_index: usize,
    accumulator: i32,
    status: Status,
}

impl BootCode {
    pub fn new(instructions_info: &[String]) -> BootCode {
        let instructions = instructions_info
            .iter()
            .map(|s| Instruction::new(s))
            .collect();

        BootCode {
            instructions: instructions,
            current_instruction_index: 0,
            accumulator: 0,
            status: Status::Initialized,
        }
    }

    pub fn execute(&mut self) {
        let mut executed_instruction_indices = HashSet::new();

        self.status = Status::Working;

        loop {
            if executed_instruction_indices.contains(&self.current_instruction_index) {
                break;
            }

            executed_instruction_indices.insert(self.current_instruction_index);

            let current_instruction;

            if let Some(instruction) = self.instructions.get(self.current_instruction_index) {
                current_instruction = instruction.clone();
            } else {
                self.status = Status::Finished;

                break;
            }

            match current_instruction.operation {
                Operation::Accumulate => self.accumulate(current_instruction.argument),
                Operation::Jump => self.jump(current_instruction.argument),
                Operation::Noop => self.no_op(),
            }
        }
    }

    pub fn execute_self_correcting(&mut self) {
        let operations_to_swap = vec![Operation::Jump, Operation::Noop].into_iter().collect();
        let indices_to_swap = self.get_jndices_of_operation_types(&operations_to_swap);

        for index_to_swap in indices_to_swap.into_iter() {
            let old_operation = self.instructions[index_to_swap].operation;

            let new_operation = match old_operation {
                Operation::Accumulate => Operation::Accumulate,
                Operation::Jump => Operation::Noop,
                Operation::Noop => Operation::Jump,
            };

            self.instructions
                .get_mut(index_to_swap)
                .unwrap()
                .change_operation(new_operation);

            self.execute();

            if self.status == Status::Finished {
                break;
            }

            self.instructions
                .get_mut(index_to_swap)
                .unwrap()
                .change_operation(old_operation);

            self.reset();
        }
    }

    pub fn get_accumulator(&self) -> i32 {
        self.accumulator
    }

    pub fn reset(&mut self) {
        self.current_instruction_index = 0;
        self.accumulator = 0;
        self.status = Status::Initialized;
    }

    fn get_jndices_of_operation_types(&self, operation_types: &HashSet<Operation>) -> Vec<usize> {
        self.instructions
            .iter()
            .enumerate()
            .filter(|(_, instruction)| operation_types.contains(&instruction.operation))
            .map(|(index, _)| index)
            .collect()
    }
    fn accumulate(&mut self, value: i32) {
        self.accumulator += value;
        self.current_instruction_index += 1;
    }

    fn jump(&mut self, value: i32) {
        self.current_instruction_index = ((self.current_instruction_index as i32) + value) as usize;
    }

    fn no_op(&mut self) {
        self.current_instruction_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 9] = [
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    #[test]
    fn test_operation_new() {
        let result_1 = Operation::new("acc");
        let result_2 = Operation::new("jmp");
        let result_3 = Operation::new("nop");

        let expected_1 = Operation::Accumulate;
        let expected_2 = Operation::Jump;
        let expected_3 = Operation::Noop;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_instruction_get_argument_value() {
        let result_1 = Instruction::get_argument_value("-99");
        let result_2 = Instruction::get_argument_value("+4");
        let result_3 = Instruction::get_argument_value("+0");

        let expected_1 = -99;
        let expected_2 = 4;
        let expected_3 = 0;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_instruction_new() {
        let result_1 = Instruction::new("acc -99");
        let result_2 = Instruction::new("jmp +4");
        let result_3 = Instruction::new("nop +0");

        let expected_1 = Instruction {
            operation: Operation::Accumulate,
            argument: -99,
        };
        let expected_2 = Instruction {
            operation: Operation::Jump,
            argument: 4,
        };
        let expected_3 = Instruction {
            operation: Operation::Noop,
            argument: 0,
        };

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_boot_code_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let result = BootCode::new(&input);

        let expected_instructions = input.iter().map(|s| Instruction::new(s)).collect();

        let expected = BootCode {
            instructions: expected_instructions,
            current_instruction_index: 0,
            accumulator: 0,
            status: Status::Initialized,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_boot_code_execute() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut boot_code = BootCode::new(&input);

        boot_code.execute();

        let result_status = boot_code.status.clone();
        let result_accumulator = boot_code.get_accumulator();

        let expected_status = Status::Working;
        let expected_accumulator = 5;

        assert_eq!(result_status, expected_status);
        assert_eq!(result_accumulator, expected_accumulator);
    }

    #[test]
    fn test_boot_code_execute_self_correcting() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut boot_code = BootCode::new(&input);

        boot_code.execute_self_correcting();

        let result_status = boot_code.status.clone();
        let result_accumulator = boot_code.get_accumulator();

        let expected_status = Status::Finished;
        let expected_accumulator = 8;

        assert_eq!(result_status, expected_status);
        assert_eq!(result_accumulator, expected_accumulator);
    }

    #[test]
    fn test_boot_code_get_jndices_of_operation_types() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let boot_code = BootCode::new(&input);

        let operation_types = vec![Operation::Jump, Operation::Noop].into_iter().collect();

        let result = boot_code.get_jndices_of_operation_types(&operation_types);

        let expected = vec![0, 2, 4, 7];

        assert_eq!(result, expected);
    }
}
