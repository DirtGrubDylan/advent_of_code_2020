use crate::util::math::lcm;

#[derive(Debug, PartialEq)]
enum ShuttleStatus {
    Running,
    OutOfService,
}

#[derive(Debug, PartialEq)]
pub struct ShuttleSchedule {
    pub id: u64,
    start_time_in_minutes: u64,
    loop_time_in_minutes: u64,
    status: ShuttleStatus,
}

impl ShuttleSchedule {
    pub fn next_closest_departure_to(&self, time_in_minutes: u64) -> Option<u64> {
        if self.status == ShuttleStatus::OutOfService {
            None
        } else if time_in_minutes % self.loop_time_in_minutes == 0 {
            Some(time_in_minutes)
        } else {
            Some(((time_in_minutes / self.loop_time_in_minutes) + 1) * self.loop_time_in_minutes)
        }
    }

    fn new(info: &str) -> ShuttleSchedule {
        if info != "x" {
            let value = info.parse().expect(info);

            ShuttleSchedule {
                id: value,
                start_time_in_minutes: 0,
                loop_time_in_minutes: value,
                status: ShuttleStatus::Running,
            }
        } else {
            ShuttleSchedule {
                id: 0,
                start_time_in_minutes: 0,
                loop_time_in_minutes: 0,
                status: ShuttleStatus::OutOfService,
            }
        }
    }

    fn is_running(&self) -> bool {
        self.status == ShuttleStatus::Running
    }
}

#[derive(Debug, PartialEq)]
pub struct ShuttleSystemCalculator {
    pub departure_time_in_minutes: u64,
    shuttles: Vec<ShuttleSchedule>,
}

impl ShuttleSystemCalculator {
    pub fn new(info: &[String]) -> ShuttleSystemCalculator {
        let err = format!("Error Input: {:?}", info);

        let departure_time_in_minutes = info.get(0).expect(&err).parse().expect(&err);

        let shuttles = info
            .get(1)
            .expect(&err)
            .split(",")
            .map(|s| ShuttleSchedule::new(s))
            .collect();

        ShuttleSystemCalculator {
            departure_time_in_minutes,
            shuttles,
        }
    }

    pub fn get_earliest_shuttle_to_airport(&self) -> Option<&ShuttleSchedule> {
        self.shuttles
            .iter()
            .filter(|shuttle| shuttle.is_running())
            .min_by_key(|shuttle| shuttle.next_closest_departure_to(self.departure_time_in_minutes))
    }

    pub fn get_timestamp_for_subsequent_departures(&self) -> Option<u64> {
        let mut result = None;

        if let Some(first_shuttle) = self.shuttles.get(0) {
            let mut loop_timestamp = first_shuttle.loop_time_in_minutes;

            let mut loop_timestamp_sum = loop_timestamp;

            for (index, next_shuttle) in self.shuttles.iter().enumerate().skip(1) {
                if !next_shuttle.is_running() {
                    continue;
                }

                let mut target_departure = loop_timestamp_sum + (index as u64);

                let mut next_earliest_departure_to_target = next_shuttle
                    .next_closest_departure_to(target_departure)
                    .unwrap();

                while next_earliest_departure_to_target != target_departure {
                    loop_timestamp_sum += loop_timestamp;

                    target_departure = loop_timestamp_sum + (index as u64);

                    next_earliest_departure_to_target = next_shuttle
                        .next_closest_departure_to(target_departure)
                        .unwrap();
                }

                result = Some(loop_timestamp_sum);
                loop_timestamp = lcm(loop_timestamp, next_shuttle.loop_time_in_minutes);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DEPART_TIME: u64 = 939;
    const TEST_SHUTTLE_RUNNING_DATA: &str = "13";
    const TEST_SHUTTLE_NOT_RUNNING_DATA: &str = "x";
    const TEST_SYSTEM_DATA: [&str; 2] = ["939", "7,13,x,x,59,x,31,19"];
    const TEST_SHUTTLES_DATA: [&str; 8] = ["7", "13", "x", "x", "59", "x", "31", "19"];

    const TEST_SYSTEMS_DATA: [[&str; 2]; 6] = [
        ["939", "7,13,x,x,59,x,31,19"],
        ["939", "17,x,13,19"],
        ["939", "67,7,59,61"],
        ["939", "67,x,7,59,61"],
        ["939", "67,7,x,59,61"],
        ["939", "1789,37,47,1889"],
    ];

    #[test]
    fn test_shuttle_service_new() {
        let result_running = ShuttleSchedule::new(TEST_SHUTTLE_RUNNING_DATA);
        let result_not_running = ShuttleSchedule::new(TEST_SHUTTLE_NOT_RUNNING_DATA);

        let expected_running = ShuttleSchedule {
            id: 13,
            start_time_in_minutes: 0,
            loop_time_in_minutes: 13,
            status: ShuttleStatus::Running,
        };
        let expected_not_running = ShuttleSchedule {
            id: 0,
            start_time_in_minutes: 0,
            loop_time_in_minutes: 0,
            status: ShuttleStatus::OutOfService,
        };

        assert_eq!(result_running, expected_running);
        assert_eq!(result_not_running, expected_not_running);
    }

    #[test]
    fn test_shuttle_service_next_closest_departure_to() {
        let result: Vec<Option<u64>> = TEST_SHUTTLES_DATA
            .iter()
            .map(|s| (ShuttleSchedule::new(s)).next_closest_departure_to(TEST_DEPART_TIME))
            .collect();

        let expected = vec![
            Some(945),
            Some(949),
            None,
            None,
            Some(944),
            None,
            Some(961),
            Some(950),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuttle_system_calculator_new() {
        let input: Vec<String> = TEST_SYSTEM_DATA.iter().map(|s| s.to_string()).collect();

        let result = ShuttleSystemCalculator::new(&input);

        let expected = ShuttleSystemCalculator {
            departure_time_in_minutes: TEST_DEPART_TIME,
            shuttles: TEST_SHUTTLES_DATA
                .iter()
                .map(|s| ShuttleSchedule::new(s))
                .collect(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shuttle_system_calculator_get_earliest_shuttle_to_airport() {
        let input: Vec<String> = TEST_SYSTEM_DATA.iter().map(|s| s.to_string()).collect();

        let system_calculator = ShuttleSystemCalculator::new(&input);

        let result = system_calculator.get_earliest_shuttle_to_airport();

        let expected = ShuttleSchedule::new("59");

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_shuttle_system_calculator_get_timestamp_for_subsequent_departures() {
        let systems: Vec<ShuttleSystemCalculator> = TEST_SYSTEMS_DATA
            .iter()
            .map(|info| {
                let input: Vec<String> = info.iter().map(|s| s.to_string()).collect();

                ShuttleSystemCalculator::new(&input)
            })
            .collect();

        let result: Vec<Option<u64>> = systems
            .iter()
            .map(|system| system.get_timestamp_for_subsequent_departures())
            .collect();

        let expected = vec![
            Some(1068781),
            Some(3417),
            Some(754018),
            Some(779210),
            Some(1261476),
            Some(1202161486),
        ];

        assert_eq!(result, expected);
    }
}
