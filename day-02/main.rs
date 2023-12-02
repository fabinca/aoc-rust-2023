/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

use std::fs;

struct Game {
    id: u8,
    red: u8,
    blue: u8,
    green: u8
}

impl Game {

    fn new(line: &str) -> Game {
        let sections: Vec<&str> = line.split(':').collect();
        let game: &str = sections[0];
        let draws: &str = sections[1];
        let game_id: String = game.chars().filter(|c| c.is_digit(10)).collect();


        let sets: Vec<&str> = draws.split(';').collect();

        for set in sets {
            println!("{}", set)
        }

        Game {
            id: game_id.parse().unwrap(),
            red: 1,
            blue: 2,
            green: 3
        }
    }

    fn get_id(&self) -> u8 {
        return self.id
    }

}

fn main() {

    let contents = fs::read_to_string("/workspaces/vscode-remote-try-rust/day2/input.txt")
    .expect("Should have been able to read the file");

    let g = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    println!("GameID: {}", g.get_id());

    let mut games: Vec<Game>;

    for line in contents.lines() {
        games.append(Game::new(line))
    }

}
