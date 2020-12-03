#[derive(Debug, PartialEq)]
pub struct TreeMap {
    map: Vec<Vec<char>>,
    dimensions: (usize, usize),
}

impl TreeMap {
    pub fn new(input: &[String]) -> TreeMap {
        let map = input.iter().map(|line| line.chars().collect()).collect();

        let dimension_x = input.get(0).unwrap().len();
        let dimension_y = input.len();

        TreeMap {
            map: map,
            dimensions: (dimension_x, dimension_y),
        }
    }

    pub fn number_of_trees_in_line(&self, x_step: usize, y_step: usize) -> u64 {
        let mut number_of_trees = 0;
        let (mut current_x, mut current_y) = (0, 0);

        while current_y < self.dimensions.1 {
            let location_marker = self
                .map
                .get(current_y)
                .expect(format!("Missing Y: {:?}", current_y).as_str())
                .get(current_x)
                .expect(format!("Missing X: {:?}", current_x).as_str());

            if *location_marker == '#' {
                number_of_trees += 1;
            }

            current_y += y_step;
            current_x = (current_x + x_step) % self.dimensions.0;
        }

        number_of_trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let input_lines = vec![String::from("..##......."), String::from("#...#...#..")];

        let result = TreeMap::new(&input_lines);

        let expected = TreeMap {
            map: vec![
                vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            ],
            dimensions: (11, 2),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_trees_in_line() {
        let input_lines = to_string_vector("test_inputs/day_3.txt").unwrap();

        let map = TreeMap::new(&input_lines);

        let result1 = map.number_of_trees_in_line(1, 1);
        let result2 = map.number_of_trees_in_line(3, 1);
        let result3 = map.number_of_trees_in_line(5, 1);
        let result4 = map.number_of_trees_in_line(7, 1);
        let result5 = map.number_of_trees_in_line(1, 2);

        let expected1 = 2;
        let expected2 = 7;
        let expected3 = 3;
        let expected4 = 4;
        let expected5 = 2;

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(result4, expected4);
        assert_eq!(result5, expected5);
    }
}
