/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

fn filter_and_convert_to_int(input_string: &str) -> u32 {
    let filtered_numbers: String = input_string.chars().filter(|c| c.is_numeric()).collect();

    if !filtered_numbers.is_empty() {
        filtered_numbers.parse::<u32>().ok().unwrap()
    } else {
        0
    }
}

impl Game {
    fn new(line: &str) -> Game {
        let sections: Vec<&str> = line.split(':').collect();
        let draws: &str = sections[1];

        let sets: Vec<&str> = draws.split(';').collect();

        let mut final_blue = 0;
        let mut final_green = 0;
        let mut final_red = 0;

        for set in sets {
            let draws: Vec<&str> = set.split(',').collect();
            for draw in draws {
                if draw.contains("blue") {
                    let blue: u32 = filter_and_convert_to_int(draw);
                    if blue > final_blue {
                        final_blue = blue
                    }
                } else if draw.contains("green") {
                    let green: u32 = filter_and_convert_to_int(draw);
                    if green > final_green {
                        final_green = green
                    }
                } else if draw.contains("red") {
                    let red: u32 = filter_and_convert_to_int(draw);
                    if red > final_red {
                        final_red = red
                    }
                }
            }
        }

        Game {
            red: final_red,
            blue: final_blue,
            green: final_green,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(contents: &str) -> u32 {
    let mut games: Vec<Game> = Vec::new();
    for line in contents.lines() {
        games.push(Game::new(line))
    }

    let mut i: u32 = 0;
    for game in games {
        let power = game.red * game.blue * game.green;
        i += power;
    }
    dbg!(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(result, 2286);
    }
}
