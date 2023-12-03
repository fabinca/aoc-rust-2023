fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let output = input.lines().map(process_line).sum::<u32>();
    output
}

fn process_line(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;
    let mut number: String = "".to_string();
    for ch in line.chars() {
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
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        );
        assert_eq!(result, 142);
    }
}
