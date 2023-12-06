fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut races: Vec<Race> = vec![];
    let times: Vec<u32> = lines[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect();
    let distances: Vec<u32> = lines[1].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect();
    for i in 0..distances.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    let mut result = 1;
    for race in races {
        let options = get_number_of_winning_options(&race);
        result *= options;
    }
    dbg!(result)
}

fn get_number_of_winning_options(race: &Race) -> u32 {
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
            "Time: 7   15  30
Distance:  9   40  200",
        );
        assert_eq!(result, 288);
    }
}
