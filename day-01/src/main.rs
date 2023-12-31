fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let output = input.lines().map(process_line).sum::<u32>();
    output
}

fn replace_written_digits(original: String) -> String {
    let replaced = original
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    dbg!(replaced)
}

fn process_line(line: &str) -> u32 {
    let replaced = replace_written_digits(line.to_string());
    let mut first_digit = None;
    let mut last_digit = None;
    let mut number: String = "".to_string();
    for ch in replaced.chars() {
        if ch.is_digit(10) {
            last_digit = Some(ch);
            if first_digit.is_none() {
                first_digit = Some(ch);
                number.push(first_digit.unwrap())
            }
        }
    }
    number.push(last_digit.unwrap());
    let integer: u32 = number.parse().unwrap();
    dbg!(integer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
