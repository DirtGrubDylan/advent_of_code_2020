use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryGame {
    said_numbers_and_last_turns: HashMap<u64, (u64, u64)>,
    turn_to_said_numbers: HashMap<u64, u64>,
    said_numbers_count: HashMap<u64, u64>,
    last_number_said: u64,
}

impl MemoryGame {
    pub fn new(starting_numbers: &[String]) -> MemoryGame {
        let mut said_numbers_and_last_turns = HashMap::new();
        let mut turn_to_said_numbers = HashMap::new();
        let mut said_numbers_count = HashMap::new();
        let mut last_number_said = 0;

        for (index, number_str) in starting_numbers.iter().enumerate() {
            let turn = (index + 1) as u64;

            last_number_said = number_str.parse().expect(number_str);

            said_numbers_and_last_turns.insert(last_number_said, (turn, turn));
            turn_to_said_numbers.insert(turn, last_number_said);
            said_numbers_count.insert(last_number_said, 1);
        }

        MemoryGame {
            said_numbers_and_last_turns,
            turn_to_said_numbers,
            said_numbers_count,
            last_number_said,
        }
    }

    pub fn play_to_turn(&mut self, turn: u64) {
        let mut turn_already_played = self.turn_to_said_numbers.contains_key(&turn);

        while !turn_already_played {
            self.play();

            turn_already_played = self.turn_to_said_numbers.contains_key(&turn);
        }
    }

    pub fn get_last_number_said(&self) -> u64 {
        self.last_number_said
    }

    fn play(&mut self) {
        let current_turn = (self.turn_to_said_numbers.len() as u64) + 1;

        let (last_number_said_first_turn, last_number_said_last_turn) = self
            .said_numbers_and_last_turns
            .get(&self.last_number_said)
            .unwrap();

        let next_number_to_say = *last_number_said_last_turn - *last_number_said_first_turn;

        *self
            .said_numbers_count
            .entry(next_number_to_say)
            .or_insert(0) += 1;

        let next_number_turns = self
            .said_numbers_and_last_turns
            .entry(next_number_to_say)
            .or_insert((current_turn, current_turn));

        *next_number_turns = (next_number_turns.1, current_turn);

        self.turn_to_said_numbers
            .insert(current_turn, next_number_to_say);

        self.last_number_said = next_number_to_say;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[&str; 3]; 7] = [
        ["0", "3", "6"],
        ["1", "3", "2"],
        ["2", "1", "3"],
        ["1", "2", "3"],
        ["2", "3", "1"],
        ["3", "2", "1"],
        ["3", "1", "2"],
    ];

    #[test]
    fn test_new() {
        let input: Vec<String> = TEST_DATA[0].iter().map(|s| s.to_string()).collect();

        let result = MemoryGame::new(&input);

        let expected = MemoryGame {
            said_numbers_and_last_turns: vec![(0, (1, 1)), (3, (2, 2)), (6, (3, 3))]
                .into_iter()
                .collect(),
            turn_to_said_numbers: vec![(1, 0), (2, 3), (3, 6)].into_iter().collect(),
            said_numbers_count: vec![(0, 1), (3, 1), (6, 1)].into_iter().collect(),
            last_number_said: 6,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_play_to_turn() {
        let input: Vec<String> = TEST_DATA[0].iter().map(|s| s.to_string()).collect();

        let mut game = MemoryGame::new(&input);

        game.play_to_turn(10);

        let expected = MemoryGame {
            said_numbers_and_last_turns: vec![
                (0, (8, 10)),
                (3, (5, 6)),
                (6, (3, 3)),
                (1, (7, 7)),
                (4, (9, 9)),
            ]
            .into_iter()
            .collect(),
            turn_to_said_numbers: vec![
                (1, 0),
                (2, 3),
                (3, 6),
                (4, 0),
                (5, 3),
                (6, 3),
                (7, 1),
                (8, 0),
                (9, 4),
                (10, 0),
            ]
            .into_iter()
            .collect(),
            said_numbers_count: vec![(0, 4), (3, 3), (6, 1), (1, 1), (4, 1)]
                .into_iter()
                .collect(),
            last_number_said: 0,
        };

        assert_eq!(game, expected);
    }

    #[test]
    fn test_play_until_turn() {
        let inputs: Vec<Vec<String>> = TEST_DATA
            .iter()
            .map(|vec| vec.iter().map(|s| s.to_string()).collect())
            .collect();

        let mut games: Vec<MemoryGame> = inputs.iter().map(|i| MemoryGame::new(&i)).collect();

        let result: Vec<u64> = games
            .iter_mut()
            .map(|game| {
                game.play_to_turn(2020);
                game.get_last_number_said()
            })
            .collect();

        let expected = vec![436, 1, 10, 27, 78, 438, 1836];

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_play_until_turn_large() {
        let input: Vec<String> = TEST_DATA[0].iter().map(|s| s.to_string()).collect();

        let mut game = MemoryGame::new(&input);

        game.play_to_turn(30_000_000);

        let result = game.get_last_number_said();

        let expected = 175_594;

        assert_eq!(result, expected);
    }
}
