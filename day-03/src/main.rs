use core::cmp::max;
use std::cmp::min;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    println!("started funciton");
    let lines: Vec<&str> = input.lines().collect();
    let mut result: u32 = 0;
    println!("lines created");
    let symbols_positions: Vec<Vec<usize>> = input.lines().map(get_symbols_positions).collect();
    let numbers_positions: Vec<Vec<(Vec<usize>, u32)>> = input
        .lines()
        .map(get_needed_symbols_positions_and_number)
        .collect();
    println!("got symbols positions");
    let lines_len = lines.len();
    for i in 0..lines_len {
        println!("parsing line {}", i);
        let from = match i {
            0 => 0,
            _ => i - 1,
        };
        let to = min(lines_len, i + 2);
        println!("{}, {}", from, to);
        for symbol_pos in &symbols_positions[i] {
            let mut numbers: Vec<u32> = vec![];
            for i in from..to {
                for (number_positions, number) in &numbers_positions[i] {
                    if number_positions.contains(&symbol_pos) {
                        numbers.push(*number);
                        println!("{}", number);
                    }
                }
            }
            if numbers.len() == 2 {
                let gear_ratio = numbers[0] * numbers[1];
                println!(
                    "gear ratio with numbers {} * {} is {}",
                    numbers[0], numbers[1], i
                );
                result += gear_ratio
            } else if numbers.len() > 2 {
                panic!("this should never happen")
            }
        }
    }
    result
}

fn get_needed_symbols_positions_and_number(line: &str) -> Vec<(Vec<usize>, u32)> {
    let line_bytes = line.as_bytes();
    let mut symbols_position_and_num: Vec<(Vec<usize>, u32)> = vec![];
    let mut i: usize = 0;
    let mut character;
    while i < line_bytes.len() {
        character = line_bytes[i] as char;
        if !character.is_digit(10) {
            i += 1;
            continue;
        }
        let mut digit: String = "".to_string();
        let mut positions: Vec<usize> = vec![];
        if i > 0 {
            positions.push(i - 1)
        }
        while character.is_digit(10) {
            positions.push(i);
            digit.push(character);
            i += 1;
            if i == line_bytes.len() {
                break;
            }
            character = line_bytes[i] as char;
        }
        positions.push(i);
        symbols_position_and_num.push((
            positions.clone(),
            digit.parse().expect("Should be a number"),
        ));
        i += 1;
    }
    dbg!(symbols_position_and_num)
}

fn get_symbols_positions(line: &str) -> Vec<usize> {
    let line_bytes = line.as_bytes();
    let mut symbols_position: Vec<usize> = vec![];
    for (i, byte) in line_bytes.iter().enumerate() {
        let character = *byte as char;
        if character == '*' {
            symbols_position.push(i);
        }
    }
    dbg!(symbols_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835);
    }
}
