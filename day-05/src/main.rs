fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Transform {
    source_range_start: i64,
    source_range_end: i64,
    transformation_add: i64,
}

impl Transform {
    fn new(line: &str) -> Transform {
        let numbers: Vec<i64> = line
            .trim()
            .split(' ')
            .filter_map(|x| x.trim().parse::<i64>().ok())
            .collect();
        Transform {
            source_range_start: numbers[1],
            source_range_end: numbers[1] + numbers[2],
            transformation_add: numbers[0] - numbers[1],
        }
    }
}

fn part1(input: &str) -> i64 {
    let sections: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    let seed_numbers: Vec<i64> = sections[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect();
    let seeds = get_seeds(&seed_numbers);

    let mut transformation: Vec<Vec<Transform>> = vec!();

    for i in 1..sections.len() {
        let lines: Vec<&str> = sections[i].lines().collect();
        let mut section_transformation: Vec<Transform> = vec![];
        for j in 1..lines.len() {
            section_transformation.push(Transform::new(lines[j]));
        }
        transformation.push(section_transformation);
    }

    let mut locations: Vec<i64> = vec![];

    for seed in seeds {
        let location = get_location(seed, &transformation);
        locations.push(location);
    }
    let mut nearest = locations[0];
    for location in locations {
        if location < nearest {
            nearest = location;
        }
    }
    dbg!(nearest)
}

fn get_seeds(seed_numbers: &Vec<i64>)-> Vec<i64>{
    let mut seeds: Vec<i64> = vec!();
    let mut i = 0;
    while i < seed_numbers.len(){
        for j in 0..seed_numbers[i+1]{
            seeds.push(seed_numbers[i] + j);
        }
        i += 2;
    }
    println!("Number of seeds: {}", seeds.len());
    seeds
}

fn get_location(seed: i64, transformation: &Vec<Vec<Transform>>) -> i64 {
    let mut transformed_number = seed;
    for section_transformation in transformation {
        transformed_number = transform(transformed_number, &section_transformation);
    }
    transformed_number
}

fn transform(original_number: i64, section_transformation: &Vec<Transform>) -> i64 {
    for transformation_option in section_transformation {
        if original_number >= transformation_option.source_range_start
            && original_number < transformation_option.source_range_end
        {
            let transformed_number = original_number + transformation_option.transformation_add;
            return transformed_number;
        }
    }
    return original_number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 46);
    }
}
