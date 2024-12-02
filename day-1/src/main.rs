use regex::Regex;
use std::fs::read_to_string;

fn part_one(input: &String) -> i32 {
    let re = Regex::new(r"(?<first>\d+)\s+(?<second>\d+)").unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.split("\n") {
        let caps = re.captures(line).unwrap();

        left.push(caps["first"].parse::<i32>().unwrap());
        right.push(caps["second"].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    result
}

fn part_two(input: &String) -> i32 {
    let re = Regex::new(r"(?<first>\d+)\s+(?<second>\d+)").unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.split("\n") {
        let caps = re.captures(line).unwrap();
        let first = &caps["first"];
        let second = &caps["second"];

        left.push(first.parse::<i32>().unwrap());
        right.push(second.parse::<i32>().unwrap());
    }

    let result = left.iter().fold(0, |mut acc, x| {
        let occurrences: i32 = right.iter().filter(|y| *y == x).count().try_into().unwrap();
        acc += x * occurrences;
        acc
    });

    result
}

fn main() {
    let input = read_to_string("./input/1.txt").expect("Failed to read input!");

    println!("Part 1: {}", part_one(&input));

    println!("Part 2: {}", part_two(&input));
}

// Testing

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
    3 4
    4 3
    2 5
    1 3
    3 9
    3 3";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(&INPUT.to_string()), 11);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(&INPUT.to_string()), 31);
    }
}
