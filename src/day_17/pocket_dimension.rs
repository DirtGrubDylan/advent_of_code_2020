use crate::util::point_3d::Point3d;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cube {
    current_active_state: bool,
}

impl Cube {
    fn new(state: char) -> Cube {
        match state {
            '.' => Cube {
                current_active_state: false,
            },
            '#' => Cube {
                current_active_state: true,
            },
            _ => panic!("Not a Valid State: {}", state),
        }
    }

    fn is_active(&self) -> bool {
        self.current_active_state
    }

    fn switch_state(&mut self) {
        self.current_active_state = !self.current_active_state;
    }
}

#[derive(Debug, PartialEq)]
pub struct PocketDimension {
    grid: HashMap<Point3d<i32>, Cube>,
}

impl PocketDimension {
    const NEARBY_POINTS_OFFSET: [(i32, i32, i32); 26] = [
        // z = -1
        (-1, -1, -1),
        (0, -1, -1),
        (1, -1, -1),
        (-1, 0, -1),
        (0, 0, -1),
        (1, 0, -1),
        (-1, 1, -1),
        (0, 1, -1),
        (1, 1, -1),
        // z = 0
        (-1, -1, 0),
        (0, -1, 0),
        (1, -1, 0),
        (-1, 0, 0),
        (1, 0, 0),
        (-1, 1, 0),
        (0, 1, 0),
        (1, 1, 0),
        // z = 1
        (-1, -1, 1),
        (0, -1, 1),
        (1, -1, 1),
        (-1, 0, 1),
        (0, 0, 1),
        (1, 0, 1),
        (-1, 1, 1),
        (0, 1, 1),
        (1, 1, 1),
    ];

    pub fn new(info: &[String]) -> PocketDimension {
        let mut grid = HashMap::new();

        for (y, line) in info.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point3d::new(x as i32, y as i32, 0);

                let cube = Cube::new(c);

                grid.insert(point, cube);
            }
        }

        PocketDimension { grid }
    }

    pub fn number_of_active_cubes(&self) -> usize {
        self.grid.values().filter(|c| c.is_active()).count()
    }

    pub fn run_cycle_to(&mut self, cycle: usize) {
        let mut current_cycle = 0;

        while current_cycle != cycle {
            self.run_cycle();

            current_cycle += 1;
        }
    }

    pub fn run_cycle(&mut self) {
        self.expand_grid();

        let mut points_to_switch_state: Vec<Point3d<i32>> = Vec::new();

        for (point, cube) in self.grid.iter() {
            let number_of_nearby_active_cubes = self.number_of_active_cubes_around(*point);

            let three_active_cubes = number_of_nearby_active_cubes == 3;
            let two_or_three_active_cubes =
                number_of_nearby_active_cubes == 2 || three_active_cubes;

            if three_active_cubes && !cube.is_active() {
                points_to_switch_state.push(*point);
            } else if !two_or_three_active_cubes && cube.is_active() {
                points_to_switch_state.push(*point);
            }
        }

        for point in points_to_switch_state {
            self.grid.get_mut(&point).unwrap().switch_state();
        }
    }

    fn make_empty_copy(&self) -> PocketDimension {
        let mut grid_copy = self.grid.clone();

        for cube in grid_copy.values_mut() {
            cube.current_active_state = false;
        }

        PocketDimension { grid: grid_copy }
    }

    fn expand_grid(&mut self) {
        let points_to_add: Vec<Point3d<i32>> = self
            .grid
            .keys()
            .flat_map(|point| self.get_nearby_locations_around(*point))
            .collect();

        for point in points_to_add {
            self.grid.entry(point).or_insert(Cube::new('.'));
        }
    }

    fn number_of_active_cubes_around(&self, point: Point3d<i32>) -> usize {
        self.get_nearby_locations_around(point)
            .iter()
            .filter(|p| self.grid.contains_key(p))
            .map(|p| self.grid.get(p).unwrap())
            .filter(|c| c.is_active())
            .count()
    }

    fn number_of_active_cubes_around_and_including(&self, point: Point3d<i32>) -> usize {
        self.get_nearby_locations_around_and_including(point)
            .iter()
            .filter(|p| self.grid.contains_key(p))
            .map(|p| self.grid.get(p).unwrap())
            .filter(|c| c.is_active())
            .count()
    }

    fn get_nearby_locations_around(&self, point: Point3d<i32>) -> Vec<Point3d<i32>> {
        Self::NEARBY_POINTS_OFFSET
            .iter()
            .map(|offset| point + Point3d::new(offset.0, offset.1, offset.2))
            .collect()
    }

    fn get_nearby_locations_around_and_including(&self, point: Point3d<i32>) -> Vec<Point3d<i32>> {
        let mut result: Vec<Point3d<i32>> = Self::NEARBY_POINTS_OFFSET
            .iter()
            .map(|offset| point + Point3d::new(offset.0, offset.1, offset.2))
            .collect();

        result.push(point);

        result
    }
}

#[derive(Debug, PartialEq)]
pub struct PocketDimensions {
    dimensions: HashMap<i32, PocketDimension>,
    current_cycle: i32,
}

impl PocketDimensions {
    pub fn new(info: &[String]) -> PocketDimensions {
        let mut dimensions = HashMap::new();
        let current_cycle = 0;

        dimensions.insert(0, PocketDimension::new(info));

        PocketDimensions {
            dimensions,
            current_cycle,
        }
    }

    pub fn number_of_active_cubes(&self) -> usize {
        self.dimensions
            .values()
            .fold(0, |acc, dimension| acc + dimension.number_of_active_cubes())
    }

    pub fn run_cycle_to(&mut self, cycle: i32) {
        while self.current_cycle != cycle {
            self.run_cycle();

            self.current_cycle += 1;
        }
    }

    pub fn run_cycle(&mut self) {
        self.expand_dimensions();

        let mut dimension_points_to_switch_state: Vec<(i32, Vec<Point3d<i32>>)> = Vec::new();

        for (w, dimension) in self.dimensions.iter() {
            let mut points_to_switch_state: Vec<Point3d<i32>> = Vec::new();

            for (point, cube) in dimension.grid.iter() {
                let number_of_nearby_active_cubes =
                    self.number_of_active_cubes_around_and_including(*point, *w);

                let three_active_cubes = number_of_nearby_active_cubes == 3;
                let two_or_three_active_cubes =
                    number_of_nearby_active_cubes == 2 || three_active_cubes;

                if three_active_cubes && !cube.is_active() {
                    points_to_switch_state.push(*point);
                } else if !two_or_three_active_cubes && cube.is_active() {
                    points_to_switch_state.push(*point);
                }
            }

            dimension_points_to_switch_state.push((*w, points_to_switch_state));
        }

        for (dimension, points_to_switch) in dimension_points_to_switch_state {
            for point in points_to_switch {
                self.dimensions
                    .get_mut(&dimension)
                    .unwrap()
                    .grid
                    .get_mut(&point)
                    .unwrap()
                    .switch_state();
            }
        }
    }

    fn expand_dimensions(&mut self) {
        for dimension in self.dimensions.values_mut() {
            dimension.expand_grid();
        }

        self.dimensions.insert(
            -self.current_cycle - 1,
            self.dimensions.get(&0).unwrap().make_empty_copy(),
        );

        self.dimensions.insert(
            self.current_cycle + 1,
            self.dimensions.get(&0).unwrap().make_empty_copy(),
        );
    }

    fn number_of_active_cubes_around_and_including(&self, point: Point3d<i32>, w: i32) -> usize {
        let mut result = 0;

        // left
        if let Some(dimension) = self.dimensions.get(&(w - 1)) {
            result += dimension.number_of_active_cubes_around_and_including(point);
        }

        // right
        if let Some(dimension) = self.dimensions.get(&(w + 1)) {
            result += dimension.number_of_active_cubes_around_and_including(point);
        }

        // self
        if let Some(dimension) = self.dimensions.get(&w) {
            result += dimension.number_of_active_cubes_around(point);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 3] = [".#.", "..#", "###"];
    const TEST_DATA_3: [&str; 5] = [".....", ".....", ".#.#.", "..##.", "..#.."];

    const TEST_POINT_2: Point3d<i32> = Point3d { x: 1, y: 1, z: -1 };

    const TEST_DATA_POINTS: [(i32, i32, i32); 25] = [
        (-1, -1, 0),
        (0, -1, 0),
        (1, -1, 0),
        (2, -1, 0),
        (3, -1, 0),
        (-1, 0, 0),
        (0, 0, 0),
        (1, 0, 0),
        (2, 0, 0),
        (3, 0, 0),
        (-1, 1, 0),
        (0, 1, 0),
        (1, 1, 0),
        (2, 1, 0),
        (3, 1, 0),
        (-1, 2, 0),
        (0, 2, 0),
        (1, 2, 0),
        (2, 2, 0),
        (3, 2, 0),
        (-1, 3, 0),
        (0, 3, 0),
        (1, 3, 0),
        (2, 3, 0),
        (3, 3, 0),
    ];
    const TEST_NEARBY_POINTS_2: [(i32, i32, i32); 26] = [
        (0, 0, -2),
        (1, 0, -2),
        (2, 0, -2),
        (0, 1, -2),
        (1, 1, -2),
        (2, 1, -2),
        (0, 2, -2),
        (1, 2, -2),
        (2, 2, -2),
        (0, 0, -1),
        (1, 0, -1),
        (2, 0, -1),
        (0, 1, -1),
        (2, 1, -1),
        (0, 2, -1),
        (1, 2, -1),
        (2, 2, -1),
        (0, 0, 0),
        (1, 0, 0),
        (2, 0, 0),
        (0, 1, 0),
        (1, 1, 0),
        (2, 1, 0),
        (0, 2, 0),
        (1, 2, 0),
        (2, 2, 0),
    ];

    #[test]
    fn test_pocket_dimension_new() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let result = PocketDimension::new(&input);

        let expected_points: Vec<Point3d<i32>> = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .into_iter()
        .map(|(x, y)| Point3d::new(x, y, 0))
        .collect();

        let expected_cubes: Vec<Cube> = TEST_DATA
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| Cube::new(c))
            .collect();

        let expected_grid: HashMap<Point3d<i32>, Cube> = expected_points
            .into_iter()
            .zip(expected_cubes.into_iter())
            .collect();

        let expected = PocketDimension {
            grid: expected_grid,
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test_pocket_dimension_number_of_active_cubes() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let dimension = PocketDimension::new(&input);

        let result = dimension.number_of_active_cubes();

        let expected = 5;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pocket_dimension_run_cycle() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let mut dimension = PocketDimension::new(&input);

        dimension.run_cycle();

        let mut result = get_z_slice(&dimension, 0);

        let expected_points: Vec<Point3d<i32>> = TEST_DATA_POINTS
            .iter()
            .map(|(x, y, z)| Point3d::new(*x, *y, *z))
            .collect();

        let expected_cubes: Vec<Cube> = TEST_DATA_3
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| Cube::new(c))
            .collect();

        let mut expected: Vec<(&Point3d<i32>, &Cube)> =
            expected_points.iter().zip(expected_cubes.iter()).collect();

        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        expected.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pocket_dimension_number_of_active_cubes_around() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let mut dimension = PocketDimension::new(&input);

        dimension.run_cycle_to(6);

        let result = dimension.number_of_active_cubes();

        let expected = 112;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pocket_dimension_get_nearby_locations_around() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let dimension = PocketDimension::new(&input);

        let result = dimension.get_nearby_locations_around(TEST_POINT_2);

        let expected: Vec<Point3d<i32>> = TEST_NEARBY_POINTS_2
            .iter()
            .map(|(x, y, z)| Point3d::new(*x, *y, *z))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pocket_dimensions_number_of_active_cubes() {
        let input = str_slice_to_string_vec(&TEST_DATA);

        let mut dimensions = PocketDimensions::new(&input);

        dimensions.run_cycle_to(6);

        let result = dimensions.number_of_active_cubes();

        let expected = 848;

        assert_eq!(result, expected);
    }

    fn str_slice_to_string_vec(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    fn get_z_slice(pocket_dimension: &PocketDimension, z: i32) -> Vec<(&Point3d<i32>, &Cube)> {
        pocket_dimension
            .grid
            .iter()
            .filter(|(p, _)| p.z == z)
            .collect()
    }
}
