use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Bitmask {
    original_info_32bit: String,
    ones: u64,
    zeros: u64,
}

impl Bitmask {
    fn new(info_32bit: &str) -> Bitmask {
        let original_info_32bit = info_32bit.to_string();

        let mut ones = 0;
        let mut zeros = 0;

        for (index, digit) in info_32bit.chars().rev().enumerate() {
            match digit {
                '1' => ones |= 1 << index,
                '0' => zeros |= 1 << index,
                'X' => (),
                _ => panic!("Bad Info in {} at: ({}, {})", info_32bit, index, digit),
            }
        }

        zeros = !zeros;

        Bitmask {
            original_info_32bit,
            ones,
            zeros,
        }
    }

    fn apply_to(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeros
    }
}

#[derive(Debug, PartialEq)]
pub struct BitmaskSystem {
    memory: HashMap<u64, u64>,
}

impl BitmaskSystem {
    pub fn new_v1(info: &[String]) -> BitmaskSystem {
        let mut bitmask = Bitmask::new(info.get(0).expect("Bad Mask Data").split_at(7).1);
        let mut memory = HashMap::new();

        for line in info.iter().skip(1) {
            if line.starts_with("mask") {
                bitmask = Bitmask::new(line.split_at(7).1);
            } else {
                let (location, value) = Self::get_memory_location_and_value(line);

                memory.insert(location, bitmask.apply_to(value));
            }
        }

        BitmaskSystem { memory }
    }

    pub fn sum_of_memory_values_with_mask(&self) -> u64 {
        self.memory.values().sum()
    }

    fn get_memory_location_and_value(mem_info: &str) -> (u64, u64) {
        let split_info: Vec<&str> = mem_info.split(" = ").collect();

        let (mut location_info, value) = (
            split_info.get(0).expect(mem_info).to_string(),
            split_info
                .get(1)
                .expect(mem_info)
                .parse()
                .expect("Could Not Value Parse"),
        );

        location_info.pop();

        let location = location_info[4..]
            .parse()
            .expect("Could Not Location Parse");

        (location, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MASK_DATA: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    const TEST_SYSTEM_DATA: [&str; 4] = [
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ];

    #[test]
    fn test_bitmask_new() {
        let result = Bitmask::new(TEST_MASK_DATA);

        let expected = Bitmask {
            original_info_32bit: TEST_MASK_DATA.to_string(),
            ones: 0b1000000,
            zeros: 0b1111111111111111111111111111111111111111111111111111111111111101,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_apply_to() {
        let bitmask = Bitmask::new(TEST_MASK_DATA);

        let result: Vec<u64> = vec![11, 101, 0]
            .iter()
            .map(|&value| bitmask.apply_to(value))
            .collect();

        let expected = vec![73, 101, 64];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_get_memory_location_and_value() {
        let result = BitmaskSystem::get_memory_location_and_value("mem[11] = 666");

        let expected = (11, 666);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_new() {
        let input: Vec<String> = TEST_SYSTEM_DATA.iter().map(|s| s.to_string()).collect();

        let result = BitmaskSystem::new_v1(&input);

        let expected = BitmaskSystem {
            memory: vec![(8, 64), (7, 101)].into_iter().collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_sum_of_memory_values_with_mask() {
        let input: Vec<String> = TEST_SYSTEM_DATA.iter().map(|s| s.to_string()).collect();

        let bitmask_system = BitmaskSystem::new_v1(&input);

        let result = bitmask_system.sum_of_memory_values_with_mask();

        let expected = 165;

        assert_eq!(result, expected);
    }
}
