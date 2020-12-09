use std::ops::Sub;
use std::hash::Hash;
use std::collections::HashSet;

pub fn find_two_values_that_sum_to<T>(values: &[T], target: T) -> Option<(T, T)>
where
    T: Sub<Output = T> + Ord + Copy + Hash,
{
    let mut result = None;

    let mut seen_values = HashSet::new();

    for &value in values {
        if target < value {
            continue;
        }

        let value_to_reach_target = target - value;

        if !seen_values.contains(&value_to_reach_target) {
            seen_values.insert(value);
        } else {
            result = Some((value, value_to_reach_target));

            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::find_two_values_that_sum_to;

    #[test]
    fn test_find_two_values_that_sum_to_some() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let target = 2020;

        let result = find_two_values_that_sum_to(&input, target);

        let expected = Some((299, 1721));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_two_values_that_sum_to_none() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let target = 2;

        let result = find_two_values_that_sum_to(&input, target);

        let expected = None;

        assert_eq!(result, expected);
    }
}
