use crate::util::location::Location;
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    pub fn new(info: &str) -> Action {
        let (action_str, value_str) = info.split_at(1);

        let value = value_str.parse().expect(info);

        match action_str {
            "N" => Action::North(value),
            "S" => Action::South(value),
            "E" => Action::East(value),
            "W" => Action::West(value),
            "L" => Action::Left(value),
            "R" => Action::Right(value),
            "F" => Action::Forward(value),
            _ => panic!("No Action For: {}", info),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    facing: Direction,
    origin: Point2d<i32>,
    location: Point2d<i32>,
    waypoint: Point2d<i32>,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            facing: Direction::East,
            origin: Point2d::new(0, 0),
            location: Point2d::new(0, 0),
            waypoint: Point2d::new(10, 1),
        }
    }

    pub fn navigate(&mut self, navigation_instructions: &[Action], use_waypoint: bool) {
        for instruction in navigation_instructions {
            match instruction {
                Action::North(value) => self.move_north(*value, use_waypoint),
                Action::South(value) => self.move_south(*value, use_waypoint),
                Action::East(value) => self.move_east(*value, use_waypoint),
                Action::West(value) => self.move_west(*value, use_waypoint),
                Action::Left(value) => self.rotate_left(*value, use_waypoint),
                Action::Right(value) => self.rotate_right(*value, use_waypoint),
                Action::Forward(value) => self.move_forward(*value, use_waypoint),
            }
        }
    }

    pub fn manhattan_distance_moved(&self) -> i32 {
        let relative_x = self.location.x - self.origin.x;
        let relative_y = self.location.y - self.origin.y;

        relative_x.abs() + relative_y.abs()
    }

    pub fn reset(&mut self) {
        self.facing = Direction::East;
        self.location = self.origin;
        self.waypoint = Point2d::new(10, 1);
    }

    fn move_north(&mut self, value: i32, use_waypoint: bool) {
        if use_waypoint {
            self.waypoint = self.waypoint.add(&Point2d::new(0, value));
        } else {
            self.location = self.location.add(&Point2d::new(0, value));
        }
    }

    fn move_south(&mut self, value: i32, use_waypoint: bool) {
        if use_waypoint {
            self.waypoint = self.waypoint.add(&Point2d::new(0, -value));
        } else {
            self.location = self.location.add(&Point2d::new(0, -value));
        }
    }

    fn move_east(&mut self, value: i32, use_waypoint: bool) {
        if use_waypoint {
            self.waypoint = self.waypoint.add(&Point2d::new(value, 0));
        } else {
            self.location = self.location.add(&Point2d::new(value, 0));
        }
    }

    fn move_west(&mut self, value: i32, use_waypoint: bool) {
        if use_waypoint {
            self.waypoint = self.waypoint.add(&Point2d::new(-value, 0));
        } else {
            self.location = self.location.add(&Point2d::new(-value, 0));
        }
    }

    fn rotate_left(&mut self, value: i32, use_waypoint: bool) {
        let normalized_value = value % 360;

        if use_waypoint {
            self.waypoint = match normalized_value {
                0 => self.waypoint,
                90 => Point2d::new(-self.waypoint.y, self.waypoint.x),
                180 => Point2d::new(-self.waypoint.x, -self.waypoint.y),
                270 => Point2d::new(self.waypoint.y, -self.waypoint.x),
                _ => panic!("Cannot Rotate to Value: {:?}", normalized_value),
            };
        } else {
            self.facing = match normalized_value {
                0 => self.facing,
                90 => match self.facing {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                },
                180 => match self.facing {
                    Direction::North => Direction::South,
                    Direction::South => Direction::North,
                    Direction::East => Direction::West,
                    Direction::West => Direction::East,
                },
                270 => match self.facing {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                },
                _ => panic!("Cannot Rotate to Value: {:?}", normalized_value),
            };
        }
    }

    fn rotate_right(&mut self, value: i32, use_waypoint: bool) {
        let normalized_value = value % 360;

        if use_waypoint {
            self.waypoint = match normalized_value {
                0 => self.waypoint,
                90 => Point2d::new(self.waypoint.y, -self.waypoint.x),
                180 => Point2d::new(-self.waypoint.x, -self.waypoint.y),
                270 => Point2d::new(-self.waypoint.y, self.waypoint.x),
                _ => panic!("Cannot Rotate to Value: {:?}", normalized_value),
            };
        } else {
            self.facing = match normalized_value {
                0 => self.facing,
                90 => match self.facing {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                },
                180 => match self.facing {
                    Direction::North => Direction::South,
                    Direction::South => Direction::North,
                    Direction::East => Direction::West,
                    Direction::West => Direction::East,
                },
                270 => match self.facing {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                },
                _ => panic!("Cannot Rotate to Value: {:?}", normalized_value),
            };
        }
    }

    fn move_forward(&mut self, value: i32, to_waypoint: bool) {
        if to_waypoint {
            self.location = self.location.add(&Point2d::new(
                value * self.waypoint.x,
                value * self.waypoint.y,
            ));
        } else {
            match self.facing {
                Direction::North => self.move_north(value, false),
                Direction::South => self.move_south(value, false),
                Direction::East => self.move_east(value, false),
                Direction::West => self.move_west(value, false),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 5] = ["F10", "N3", "F7", "R90", "F11"];

    #[test]
    fn test_action_new() {
        let results: Vec<Action> = TEST_DATA.iter().map(|s| Action::new(s)).collect();

        let expected = vec![
            Action::Forward(10),
            Action::North(3),
            Action::Forward(7),
            Action::Right(90),
            Action::Forward(11),
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_ship_navigate() {
        let actions: Vec<Action> = TEST_DATA.iter().map(|s| Action::new(s)).collect();

        let mut ship = Ship::new();

        ship.navigate(&actions, false);

        let expected = Ship {
            facing: Direction::South,
            origin: Point2d::new(0, 0),
            location: Point2d::new(17, -8),
            waypoint: Point2d::new(10, 1),
        };

        assert_eq!(ship, expected);
    }

    #[test]
    fn test_ship_manhattan_distance_moved() {
        let actions: Vec<Action> = TEST_DATA.iter().map(|s| Action::new(s)).collect();

        let mut ship = Ship::new();

        ship.navigate(&actions, false);

        let result = ship.manhattan_distance_moved();

        let expected = 25;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ship_manhattan_distance_moved_use_waypoint() {
        let actions: Vec<Action> = TEST_DATA.iter().map(|s| Action::new(s)).collect();

        let mut ship = Ship::new();

        ship.navigate(&actions, true);

        let result = ship.manhattan_distance_moved();

        let expected = 286;

        assert_eq!(result, expected);
    }
}
