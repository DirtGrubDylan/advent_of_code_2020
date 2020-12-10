use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Adapter {
    joltage_rating: i32,
}

impl Adapter {
    fn new(joltage_rating: &str) -> Adapter {
        Adapter {
            joltage_rating: joltage_rating.parse().expect(joltage_rating),
        }
    }

    fn get_joltage_rating(&self) -> i32 {
        self.joltage_rating
    }
}

#[derive(Debug, PartialEq)]
struct AdapterArray {
    adapters: Vec<Adapter>,
}

impl AdapterArray {
    fn new(joltage_ratings: &[String]) -> AdapterArray {
        let temp_vec: Vec<Adapter> = joltage_ratings.iter().map(|s| Adapter::new(s)).collect();

        AdapterArray { adapters: temp_vec }
    }

    fn insert_adapter(&mut self, adapter: Adapter) {
        self.adapters.push(adapter);
    }

    fn get_joltage_differences_using_all_adapters(&self) -> HashMap<i32, i32> {
        let mut adapters_sorted = self.adapters.clone();

        adapters_sorted.sort();

        let mut joltage_differences = HashMap::new();

        let mut current_joltage = adapters_sorted
            .get(0)
            .unwrap_or(&Adapter::new("0"))
            .get_joltage_rating();

        for adapter in &adapters_sorted[1..] {
            let joltage_difference = adapter.get_joltage_rating() - current_joltage;

            *joltage_differences.entry(joltage_difference).or_insert(0) += 1;

            current_joltage = adapter.get_joltage_rating();
        }

        joltage_differences
    }

    fn number_of_unique_adapter_combinations(&self) -> u64 {
        let mut adapters_sorted = self.adapters.clone();

        adapters_sorted.sort();

        let mut combinations_up_to: HashMap<i32, u64> = HashMap::new();

        let largest_joltage_rating = self.get_largest_joltage_rating();
        let smallest_joltage_rating = self.get_smallest_joltage_rating();

        combinations_up_to.insert(smallest_joltage_rating, 1);

        for adapter in &adapters_sorted[1..] {
            let one_joltage_rating_behind = adapter.get_joltage_rating() - 1;
            let two_joltage_rating_behind = adapter.get_joltage_rating() - 2;
            let three_joltage_rating_behind = adapter.get_joltage_rating() - 3;

            let combinations_up_to_one_behind = *combinations_up_to
                .get(&one_joltage_rating_behind)
                .unwrap_or(&0);
            let combinations_up_to_two_behind = *combinations_up_to
                .get(&two_joltage_rating_behind)
                .unwrap_or(&0);
            let combinations_up_to_three_behind = *combinations_up_to
                .get(&three_joltage_rating_behind)
                .unwrap_or(&0);

            combinations_up_to.insert(
                adapter.get_joltage_rating(),
                combinations_up_to_one_behind
                    + combinations_up_to_two_behind
                    + combinations_up_to_three_behind,
            );
        }

        *combinations_up_to
            .get(&largest_joltage_rating)
            .unwrap_or(&0)
    }

    fn get_largest_joltage_rating(&self) -> i32 {
        self.adapters
            .iter()
            .max()
            .unwrap_or(&Adapter::new("0"))
            .get_joltage_rating()
    }

    fn get_smallest_joltage_rating(&self) -> i32 {
        self.adapters
            .iter()
            .min()
            .unwrap_or(&Adapter::new("0"))
            .get_joltage_rating()
    }
}

#[derive(Debug, PartialEq)]
pub struct Device {
    adapter_array: AdapterArray,
    built_in_joltage_difference: i32,
}

impl Device {
    pub fn new(adapters_joltage_rating: &[String], built_in_joltage_difference: i32) -> Device {
        let mut adapter_array = AdapterArray::new(adapters_joltage_rating);

        adapter_array.insert_adapter(Adapter::new("0"));

        let built_in_joltage_rating =
            adapter_array.get_largest_joltage_rating() + built_in_joltage_difference;

        adapter_array.insert_adapter(Adapter {
            joltage_rating: built_in_joltage_rating,
        });

        Device {
            adapter_array,
            built_in_joltage_difference,
        }
    }

    pub fn get_joltage_differences_using_all_adapters(&self) -> HashMap<i32, i32> {
        self.adapter_array
            .get_joltage_differences_using_all_adapters()
    }

    pub fn number_of_unique_adapter_combinations(&self) -> u64 {
        self.adapter_array.number_of_unique_adapter_combinations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file_reader::to_string_vector;

    const TEST_DATA: [&str; 11] = ["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];

    #[test]
    fn test_adapter_array_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let result = AdapterArray::new(&input);

        let expected = AdapterArray {
            adapters: vec![
                Adapter { joltage_rating: 16 },
                Adapter { joltage_rating: 10 },
                Adapter { joltage_rating: 15 },
                Adapter { joltage_rating: 5 },
                Adapter { joltage_rating: 1 },
                Adapter { joltage_rating: 11 },
                Adapter { joltage_rating: 7 },
                Adapter { joltage_rating: 19 },
                Adapter { joltage_rating: 6 },
                Adapter { joltage_rating: 12 },
                Adapter { joltage_rating: 4 },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_adapter_array_get_joltage_differences_using_all_adapters_simple() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let adapter_array = AdapterArray::new(&input);

        let result = adapter_array.get_joltage_differences_using_all_adapters();

        let expected = vec![(1, 6), (3, 4)].into_iter().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_adapter_array_get_joltage_differences_using_all_adapters_complex() {
        let input = to_string_vector("test_inputs/day_10.txt").unwrap();

        let adapter_array = AdapterArray::new(&input);

        let result = adapter_array.get_joltage_differences_using_all_adapters();

        let expected = vec![(1, 21), (3, 9)].into_iter().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_adapter_array_number_of_unique_adapter_combinations_simple() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let adapter_array = AdapterArray::new(&input);

        let result = adapter_array.number_of_unique_adapter_combinations();

        let expected = 8;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_device_get_joltage_differences_using_all_adapters() {
        let input = to_string_vector("test_inputs/day_10.txt").unwrap();

        let device = Device::new(&input, 3);

        let result = device.get_joltage_differences_using_all_adapters();

        let expected = vec![(1, 22), (3, 10)].into_iter().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_device_number_of_unique_adapter_combinations() {
        let input = to_string_vector("test_inputs/day_10.txt").unwrap();

        let device = Device::new(&input, 3);

        let result = device.number_of_unique_adapter_combinations();

        let expected = 19208;

        assert_eq!(result, expected);
    }
}
