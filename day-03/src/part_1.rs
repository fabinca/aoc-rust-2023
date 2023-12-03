fn main() {
    let input= include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    println!("started funciton");
    let lines: Vec<&str> = input.lines().collect();
    let mut result:u32 = 0;
    println!("lines created");
    let symbols_positions: Vec<Vec<usize>> = input.lines().map(get_symbols_positions).collect();
    println!("got symbold positions");
    for i in 0..lines.len() {
        println!("parsing line {}", i );
        let needed_symbols_positions: Vec<(Vec<usize>, u32)> = get_needed_symbols_positions_and_number(lines[i]);
        let mut merged_symbols_positions: Vec<usize> = symbols_positions[i].clone();
        if i > 0 {
            merged_symbols_positions.append(&mut symbols_positions[i - 1].clone());
        }
        if i < lines.len() - 1 {
            merged_symbols_positions.append(&mut symbols_positions[i + 1].clone());
        }

        for (needed_positions, number) in needed_symbols_positions {
            println!("checking if number {} is adjacent to a symbol", number );
            for position in needed_positions {
                if merged_symbols_positions.contains(&position) {
                    result += number;
                    break;
                }
            }
        }

    }

    result
}

fn get_needed_symbols_positions_and_number(line: &str) -> Vec::<(Vec::<usize>, u32)>{
    let line_bytes = line.as_bytes();
    let mut symbols_position_and_num: Vec::<(Vec::<usize>, u32)> = vec!();
    let mut i:usize = 0;
    let mut character;
    while i < line_bytes.len(){
        println!("{}", i);
        character = line_bytes[i] as char;
        if !character.is_digit(10){
            i += 1;
            continue;
        }
        let mut digit: String = "".to_string();
        let mut positions: Vec<usize> = vec!();
        if i > 0 {
            positions.push(i - 1)
        }
        while character.is_digit(10) {
            println!("in while loop {}", i);
            positions.push(i);
            digit.push(character);
            i+= 1;
            if i == line_bytes.len() {
                break;
            }
            character = line_bytes[i] as char;
        }
        positions.push(i);
        println!("Found number: {}", digit);
        symbols_position_and_num.push((positions.clone(), digit.parse().expect("Should be a number")));
        i+=1;
    }
    symbols_position_and_num
}

fn get_symbols_positions(line: &str) -> Vec<usize> {
    let line_bytes = line.as_bytes();
    let mut symbols_position: Vec<usize> = vec!();
    for (i, byte) in line_bytes.iter().enumerate(){
        let character = *byte as char;
        if character != '.' && !character.is_digit(10){
            symbols_position.push(i);}
    }
    symbols_position
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}