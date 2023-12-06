/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let sections: Vec<&str> = line.split(':').collect();
        let game: &str = sections[0];
        let numbers: &str = sections[1];
        let card_id: String = game.chars().filter(|c| c.is_ascii_digit()).collect();

        let sets: Vec<&str> = numbers.split('|').collect();

        let winning: Vec<u32> = sets[0]
            .replace("  ",  " ")
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let mine: Vec<u32> = sets[1]
            .replace("  ",  " ")
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Card {
            id: card_id.parse().expect("Should be a number"),
            winning_numbers: winning,
            my_numbers: mine,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(contents: &str) -> u32 {
    let output = contents.lines().map(process_line).sum::<u32>();
    dbg!(output)
}

fn process_line(line: &str) -> u32 {
    let card: Card = Card::new(line);
    let mut result = 0;
    for my_number in card.my_numbers {
        if card.winning_numbers.contains(&my_number) {
            match result {
                0 => result = 1,
                _ => result *= 2,
            };
        }
    }
    dbg!(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        );
        assert_eq!(result, 13);
    }
}
