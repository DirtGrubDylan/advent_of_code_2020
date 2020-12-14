use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Bitmask {
    original_info_32bit: String,
    ones: u64,
    zeros: u64,
    floats: HashSet<u64>,
}

impl Bitmask {
    fn new(info_32bit: &str, include_floats: bool) -> Bitmask {
        let original_info_32bit = info_32bit.to_string();

        let mut ones = 0;
        let mut zeros = 0;
        let mut floats = HashSet::new();

        if include_floats {
            floats.insert(0);
        }

        for (index, digit) in info_32bit.chars().rev().enumerate() {
            match digit {
                '1' => ones |= 1 << index,
                '0' => zeros |= 1 << index,
                'X' => {
                    if include_floats {
                        let new_float_value = 1 << index;

                        let combined_values: Vec<u64> =
                            floats.iter().map(|float| float | new_float_value).collect();

                        for value in combined_values {
                            floats.insert(value);
                        }
                    }
                }
                _ => panic!("Bad Info in {} at: ({}, {})", info_32bit, index, digit),
            }
        }

        zeros = !zeros;

        Bitmask {
            original_info_32bit,
            ones,
            zeros,
            floats,
        }
    }

    fn apply_to_v1(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeros
    }

    fn apply_to_v2(&self, value: u64) -> HashSet<u64> {
        let temp_value = (value | self.ones) & !self.floats.iter().max().unwrap_or(&0);

        self.floats
            .iter()
            .map(|float_value| temp_value | float_value)
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct BitmaskSystem {
    memory: HashMap<u64, u64>,
}

impl BitmaskSystem {
    pub fn new_v1(info: &[String]) -> BitmaskSystem {
        let mut bitmask = Bitmask::new(info.get(0).expect("Bad Mask Data").split_at(7).1, false);
        let mut memory = HashMap::new();

        for line in info.iter().skip(1) {
            if line.starts_with("mask") {
                bitmask = Bitmask::new(line.split_at(7).1, false);
            } else {
                let (location, value) = Self::get_memory_location_and_value(line);

                memory.insert(location, bitmask.apply_to_v1(value));
            }
        }

        BitmaskSystem { memory }
    }

    pub fn new_v2(info: &[String]) -> BitmaskSystem {
        let mut bitmask = Bitmask::new(info.get(0).expect("Bad Mask Data").split_at(7).1, true);
        let mut memory = HashMap::new();

        for line in info.iter().skip(1) {
            if line.starts_with("mask") {
                bitmask = Bitmask::new(line.split_at(7).1, true);
            } else {
                let (location, value) = Self::get_memory_location_and_value(line);

                for masked_location in bitmask.apply_to_v2(location).iter() {
                    memory.insert(*masked_location, value);
                }
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

    const TEST_MASK_DATA_V1: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    const TEST_SYSTEM_DATA_V1: [&str; 4] = [
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ];

    const TEST_MASK_DATA_V2_1: &str = "000000000000000000000000000000X1001X";
    const TEST_MASK_DATA_V2_2: &str = "00000000000000000000000000000000X0XX";
    const TEST_SYSTEM_DATA_V2: [&str; 4] = [
        "mask = 000000000000000000000000000000X1001X",
        "mem[42] = 100",
        "mask = 00000000000000000000000000000000X0XX",
        "mem[26] = 1",
    ];

    #[test]
    fn test_bitmask_new_v1() {
        let result = Bitmask::new(TEST_MASK_DATA_V1, false);

        let expected = Bitmask {
            original_info_32bit: TEST_MASK_DATA_V1.to_string(),
            ones: 0b1000000,
            zeros: 0b1111111111111111111111111111111111111111111111111111111111111101,
            floats: HashSet::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_new_v2() {
        let result_1 = Bitmask::new(TEST_MASK_DATA_V2_1, true);
        let result_2 = Bitmask::new(TEST_MASK_DATA_V2_2, true);

        let expected_1 = Bitmask {
            original_info_32bit: TEST_MASK_DATA_V2_1.to_string(),
            ones: 0b10010,
            zeros: 0b1111111111111111111111111111000000000000000000000000000000110011,
            floats: vec![0b000000, 0b100000, 0b000001, 0b100001]
                .into_iter()
                .collect(),
        };
        let expected_2 = Bitmask {
            original_info_32bit: TEST_MASK_DATA_V2_2.to_string(),
            ones: 0,
            zeros: 0b1111111111111111111111111111000000000000000000000000000000001011,
            floats: vec![
                0b0000, 0b1000, 0b0010, 0b0001, 0b1010, 0b1001, 0b0011, 0b1011,
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_bitmask_apply_to_v1() {
        let bitmask = Bitmask::new(TEST_MASK_DATA_V1, false);

        let result: Vec<u64> = vec![11, 101, 0]
            .iter()
            .map(|&value| bitmask.apply_to_v1(value))
            .collect();

        let expected = vec![73, 101, 64];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_apply_to_v2() {
        let bitmasks = vec![
            Bitmask::new(TEST_MASK_DATA_V2_1, true),
            Bitmask::new(TEST_MASK_DATA_V2_2, true),
        ];

        let result: Vec<HashSet<u64>> = bitmasks
            .iter()
            .zip(vec![42, 26].iter())
            .map(|(bitmask, &value)| bitmask.apply_to_v2(value))
            .collect();

        let expected = vec![
            vec![26, 27, 58, 59].into_iter().collect(),
            vec![16, 17, 18, 19, 24, 25, 26, 27].into_iter().collect(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_get_memory_location_and_value() {
        let result = BitmaskSystem::get_memory_location_and_value("mem[11] = 666");

        let expected = (11, 666);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_new_v1() {
        let input: Vec<String> = TEST_SYSTEM_DATA_V1.iter().map(|s| s.to_string()).collect();

        let result = BitmaskSystem::new_v1(&input);

        let expected = BitmaskSystem {
            memory: vec![(8, 64), (7, 101)].into_iter().collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_new_v2() {
        let input: Vec<String> = TEST_SYSTEM_DATA_V2.iter().map(|s| s.to_string()).collect();

        let result = BitmaskSystem::new_v2(&input);

        let expected = BitmaskSystem {
            memory: vec![
                (58, 100),
                (59, 100),
                (16, 1),
                (17, 1),
                (18, 1),
                (19, 1),
                (24, 1),
                (25, 1),
                (26, 1),
                (27, 1),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_sum_of_memory_values_with_mask_v1() {
        let input: Vec<String> = TEST_SYSTEM_DATA_V1.iter().map(|s| s.to_string()).collect();

        let bitmask_system = BitmaskSystem::new_v1(&input);

        let result = bitmask_system.sum_of_memory_values_with_mask();

        let expected = 165;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitmask_system_sum_of_memory_values_with_mask_v2() {
        let input: Vec<String> = TEST_SYSTEM_DATA_V2.iter().map(|s| s.to_string()).collect();

        let bitmask_system = BitmaskSystem::new_v2(&input);

        let result = bitmask_system.sum_of_memory_values_with_mask();

        let expected = 208;

        assert_eq!(result, expected);
    }
}
