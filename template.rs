fn main() {
    let input= include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let output = input.lines().map(process_line).sum::<u32>();
    output
}

fn process_line(line: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1("");
        assert_eq!(result, "".to_string());
    }
}