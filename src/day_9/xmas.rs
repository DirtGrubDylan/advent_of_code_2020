use std::ops::Range;

use crate::util::sum_finder::find_two_values_that_sum_to;

#[derive(Debug, PartialEq)]
pub struct Xmas {
    preamble_length: usize,
    data: Vec<u64>,
}

impl Xmas {
    pub fn new(data: &[String], preamble_length: usize) -> Xmas {
        Xmas {
            preamble_length,
            data: data.iter().map(|s| s.parse().expect(s)).collect(),
        }
    }

    pub fn find_first_invalid_element(&self) -> Option<u64> {
        let mut result = None;

        let mut window_start = self.preamble_length;
        let mut window_end = window_start + self.preamble_length;
        let mut target_index = window_end;

        while let Some(temp_data_slice) = self.data.get(window_start..window_end) {
            if let Some(&target) = self.data.get(target_index) {
                let find_sum_result = find_two_values_that_sum_to(temp_data_slice, target);

                if find_sum_result.is_none() {
                    result = Some(target);

                    break;
                }
            } else {
                break;
            }

            window_start += 1;
            window_end += 1;
            target_index += 1;
        }

        result
    }

    pub fn find_encryption_weakness(&self) -> Option<u64> {
        let first_invalid_element = self.find_first_invalid_element();
        let mut result = None;

        if let Some(invalid_value) = first_invalid_element {
            let mut result_slice_range: Option<Range<usize>> = None;

            for (outer_index, &sum_start) in self.data.iter().enumerate() {
                if outer_index == self.data.len() - 1 {
                    break;
                }

                let mut temp_sum = sum_start;

                for (mut inner_index, value) in self.data[(outer_index + 1)..].iter().enumerate() {
                    temp_sum += value;
                    inner_index += outer_index + 1;

                    if invalid_value < temp_sum {
                        break;
                    } else if invalid_value == temp_sum {
                        result_slice_range = Some(outer_index..inner_index);
                    }
                }

                if result_slice_range.is_some() {
                    break;
                }
            }

            if let Some(range) = result_slice_range {
                let result_slice = self.data.get(range).unwrap();

                result =
                    Some(result_slice.iter().min().unwrap() + result_slice.iter().max().unwrap());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 20] = [
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    #[test]
    fn test_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let result = Xmas::new(&input, 5);

        let expected = Xmas {
            preamble_length: 5,
            data: TEST_DATA.iter().map(|s| s.parse().unwrap()).collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_first_invalid_element() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let xmas = Xmas::new(&input, 5);

        let result = xmas.find_first_invalid_element();

        let expected = Some(127);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_encryption_weakness() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let xmas = Xmas::new(&input, 5);

        let result = xmas.find_encryption_weakness();

        let expected = Some(62);

        assert_eq!(result, expected);
    }
}
