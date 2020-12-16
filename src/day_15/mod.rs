mod memory_game;

use memory_game::MemoryGame;

pub fn run_day_15() {
    let input: Vec<String> = vec![
        String::from("1"),
        String::from("20"),
        String::from("8"),
        String::from("12"),
        String::from("0"),
        String::from("14"),
    ];

    let mut game = MemoryGame::new(&input);

    game.play_to_turn(2020);

    let part_1 = game.get_last_number_said();

    game.play_to_turn(30_000_000);

    let part_2 = game.get_last_number_said();
    println!("Day 15 Part 1: {}", part_1);
    println!("Day 15 Part 2: {}", part_2);
}
