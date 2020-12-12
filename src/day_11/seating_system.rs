use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LayoutObject {
    Floor,
    OccupiedSeat,
    EmptySeat,
}

impl LayoutObject {
    fn new(info: char) -> LayoutObject {
        match info {
            '.' => LayoutObject::Floor,
            'L' => LayoutObject::EmptySeat,
            '#' => LayoutObject::OccupiedSeat,
            _ => panic!("No Object For: {}", info),
        }
    }

    fn switch_occupancy(&self) -> LayoutObject {
        match self {
            LayoutObject::Floor => LayoutObject::Floor,
            LayoutObject::EmptySeat => LayoutObject::OccupiedSeat,
            LayoutObject::OccupiedSeat => LayoutObject::EmptySeat,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SeatingSystem {
    layout: HashMap<(i32, i32), LayoutObject>,
    size: (usize, usize),
    current_cycle: u32,
    is_stable: bool,
    original_input: Vec<String>,
}

impl SeatingSystem {
    const SURROUNDING_LOCATIONS: [(i32, i32); 8] = [
        (0, -1),  // up
        (1, -1),  // up right
        (1, 0),   // right
        (1, 1),   // right down
        (0, 1),   // down
        (-1, 1),  // down left
        (-1, 0),  // left
        (-1, -1), // up left
    ];

    pub fn new(info: &[String]) -> SeatingSystem {
        let mut temp = HashMap::new();

        let size = (info.get(0).unwrap_or(&String::new()).len(), info.len());

        for (row_index, row) in info.iter().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                let point = (col_index as i32, row_index as i32);

                let layout_object = LayoutObject::new(col);

                temp.insert(point, layout_object);
            }
        }

        SeatingSystem {
            layout: temp,
            size: size,
            current_cycle: 0,
            is_stable: false,
            original_input: info.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn number_of_occupied_seats(&self) -> usize {
        self.layout
            .values()
            .filter(|object| **object == LayoutObject::OccupiedSeat)
            .count()
    }

    pub fn simulate_until_stable_with_los(&mut self) {
        while !self.is_stable {
            self.simulate_cycle(5, true);
        }
    }

    pub fn simulate_until_stable(&mut self) {
        while !self.is_stable {
            self.simulate_cycle(4, false);
        }
    }

    pub fn reset(&mut self) {
        *self = SeatingSystem::new(&self.original_input);
    }

    fn simulate_cycle(&mut self, occupancy_threshold: usize, los: bool) {
        let old_layout = self.layout.clone();

        let mut seats_changed = 0;

        for row in 0..self.size.1 {
            for col in 0..self.size.0 {
                let current_location = (col as i32, row as i32);

                let seat_changed = self.simulate_seat_change(
                    current_location,
                    &old_layout,
                    occupancy_threshold,
                    los,
                );

                if seat_changed {
                    seats_changed += 1;
                }
            }
        }

        if seats_changed == 0 {
            self.is_stable = true;
        } else {
            self.current_cycle += 1;
        }
    }

    fn simulate_seat_change(
        &mut self,
        location: (i32, i32),
        old_layout: &HashMap<(i32, i32), LayoutObject>,
        occupancy_threshold: usize,
        los: bool,
    ) -> bool {
        let temp = format!("Location Err: {:?}\nSize: {:?}", location, self.size);
        let object = old_layout.get(&location).expect(&temp).clone();

        let surrounding_seats = if los {
            self.get_objects_in_los(location, old_layout)
        } else {
            self.get_surrounding_seats(location, &old_layout)
        };

        let surrounding_is_empty = surrounding_seats
            .iter()
            .all(|&o| (o == LayoutObject::Floor || o == LayoutObject::EmptySeat));

        let surrounding_above_threshold = occupancy_threshold
            <= surrounding_seats
                .iter()
                .filter(|o| **o == LayoutObject::OccupiedSeat)
                .count();

        self.change_seats(
            location,
            object,
            surrounding_is_empty,
            surrounding_above_threshold,
        )
    }

    fn get_surround_locations(&self, location: &(i32, i32)) -> Vec<(i32, i32)> {
        Self::SURROUNDING_LOCATIONS
            .iter()
            .map(|p| (p.0 + location.0, p.1 + location.1))
            .collect()
    }

    fn get_surrounding_seats(
        &self,
        location: (i32, i32),
        layout: &HashMap<(i32, i32), LayoutObject>,
    ) -> Vec<LayoutObject> {
        self.get_surround_locations(&location)
            .iter()
            .filter(|p| layout.contains_key(p))
            .map(|p| layout.get(p).unwrap().clone())
            .collect()
    }

    fn get_objects_in_los(
        &self,
        location: (i32, i32),
        layout: &HashMap<(i32, i32), LayoutObject>,
    ) -> Vec<LayoutObject> {
        Self::SURROUNDING_LOCATIONS
            .iter()
            .map(|&scale| self.get_first_seat_location_in_line(location, scale, layout))
            .filter(|item| item.is_some())
            .map(|item| layout.get(&item.unwrap()).unwrap().clone())
            .collect()
    }

    fn get_first_seat_location_in_line(
        &self,
        location: (i32, i32),
        scale: (i32, i32),
        layout: &HashMap<(i32, i32), LayoutObject>,
    ) -> Option<(i32, i32)> {
        let next = (location.0 + scale.0, location.1 + scale.1);

        if let Some(object) = layout.get(&next) {
            match object {
                LayoutObject::EmptySeat | LayoutObject::OccupiedSeat => Some(next),
                LayoutObject::Floor => self.get_first_seat_location_in_line(next, scale, layout),
            }
        } else {
            None
        }
    }

    fn change_seats(
        &mut self,
        location: (i32, i32),
        object: LayoutObject,
        surrounding_is_empty: bool,
        surrounding_above_threshold: bool,
    ) -> bool {
        if (object == LayoutObject::EmptySeat) && surrounding_is_empty {
            *self.layout.get_mut(&location).unwrap() = object.switch_occupancy();

            true
        } else if (object == LayoutObject::OccupiedSeat) && surrounding_above_threshold {
            *self.layout.get_mut(&location).unwrap() = object.switch_occupancy();

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 10] = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    const TEST_DATA_THIRD_CYCLE: [&str; 10] = [
        "#.##.L#.##",
        "#L###LL.L#",
        "L.#.#..#..",
        "#L##.##.L#",
        "#.##.LL.LL",
        "#.###L#.##",
        "..#.#.....",
        "#L######L#",
        "#.LL###L.L",
        "#.#L###.##",
    ];

    const TEST_DATA_LOS: [&str; 9] = [
        ".......#.",
        "...#.....",
        ".#.......",
        ".........",
        "..#L....#",
        "....#....",
        ".........",
        "#........",
        "...#.....",
    ];

    const TEST_DATA_NO_LOS: [&str; 7] = [
        ".##.##.", "#.#.#.#", "##...##", "...L...", "##...##", "#.#.#.#", ".##.##.",
    ];

    #[test]
    fn test_layout_object_new() {
        let result_1 = LayoutObject::new('.');
        let result_2 = LayoutObject::new('L');
        let result_3 = LayoutObject::new('#');

        let expected_1 = LayoutObject::Floor;
        let expected_2 = LayoutObject::EmptySeat;
        let expected_3 = LayoutObject::OccupiedSeat;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_layout_object_switch_occupancy() {
        let floor = LayoutObject::new('.');
        let empty_seat = LayoutObject::new('L');
        let occupied_seat = LayoutObject::new('#');

        let result_1 = floor.switch_occupancy();
        let result_2 = occupied_seat.switch_occupancy();
        let result_3 = empty_seat.switch_occupancy();

        let expected_1 = LayoutObject::Floor;
        let expected_2 = LayoutObject::EmptySeat;
        let expected_3 = LayoutObject::OccupiedSeat;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_seating_system_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let seating_system = SeatingSystem::new(&input);

        let result = seating_system.layout.len();

        let expected = 100;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_seating_system_number_of_occupied_seats() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut seating_system = SeatingSystem::new(&input);

        let result_1 = seating_system.number_of_occupied_seats();

        seating_system.simulate_cycle(4, false);

        let result_2 = seating_system.number_of_occupied_seats();

        let expected_1 = 0;
        let expected_2 = 71;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_seating_system_number_of_cycles_until_stable() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut seating_system = SeatingSystem::new(&input);

        seating_system.simulate_until_stable();

        let expected = 5;

        assert_eq!(seating_system.current_cycle, expected);
        assert!(seating_system.is_stable);
    }

    #[test]
    fn test_seating_system_simulate_cycle() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();
        let input_2: Vec<String> = TEST_DATA_THIRD_CYCLE
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut seating_system = SeatingSystem::new(&input);

        seating_system.simulate_cycle(4, false);
        seating_system.simulate_cycle(4, false);
        seating_system.simulate_cycle(4, false);

        let expected_system = SeatingSystem {
            current_cycle: 3,
            is_stable: false,
            original_input: input,
            ..SeatingSystem::new(&input_2)
        };

        assert_eq!(seating_system, expected_system);
    }

    #[test]
    fn test_seating_system_get_surrounding_locations() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let seating_system = SeatingSystem::new(&input);

        let result = seating_system.get_surround_locations(&(9, 9));

        let expected = vec![
            (9, 8),
            (10, 8),
            (10, 9),
            (10, 10),
            (9, 10),
            (8, 10),
            (8, 9),
            (8, 8),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_seating_system_get_objects_in_los_empty() {
        let input: Vec<String> = TEST_DATA_NO_LOS.iter().map(|s| s.to_string()).collect();

        let seating_system = SeatingSystem::new(&input);

        let old_layout = seating_system.layout.clone();

        let location = (3, 3);

        let result = seating_system.get_objects_in_los(location, &old_layout);

        assert!(result.is_empty());
    }

    #[test]
    fn test_seating_system_get_first_seat_location_in_line() {
        let input: Vec<String> = TEST_DATA_LOS.iter().map(|s| s.to_string()).collect();

        let seating_system = SeatingSystem::new(&input);

        let old_layout = seating_system.layout.clone();

        let location = (3, 4);
        let scale = (1, -1); // up right;

        let result = seating_system.get_first_seat_location_in_line(location, scale, &old_layout);

        let expected = Some((7, 0));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_seating_system_number_of_cycles_until_stable_los() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut seating_system = SeatingSystem::new(&input);

        seating_system.simulate_until_stable_with_los();

        let expected = 6;

        assert_eq!(seating_system.current_cycle, expected);
        assert!(seating_system.is_stable);
    }
}
