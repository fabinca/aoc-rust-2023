fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines[0].split(':').collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance: u64 = lines[1].split(':').collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let race = Race {
        time: time,
        distance: distance,
    };
    let options = get_number_of_winning_options(&race);
    dbg!(options)
}

fn get_number_of_winning_options(race: &Race) -> u64 {
    let mut number_of_winning_options = 0;
    let race_time = race.time;
    for i in 1..race_time {
        let speed = i;
        let moving_time = race_time - i;
        let distance = speed * moving_time;
        if distance > race.distance {
            number_of_winning_options += 1;
        }
    }
    dbg!(number_of_winning_options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "Time: 7 15 30
Distance:  9 40 200",
        );
        assert_eq!(result, 71503);
    }
}
