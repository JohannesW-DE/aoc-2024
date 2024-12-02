use std::fs::read_to_string;

fn get_diffs(arr: Vec<i32>) -> Vec<i32> {
    return arr
        .iter()
        .enumerate()
        .map(|(i, &value)| match i {
            0 => None,
            _ => Some(value - arr[i - 1]),
        })
        .filter_map(|x| x)
        .collect();
}

fn is_safe(arr: Vec<i32>) -> bool {
    return arr.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
        && (arr.iter().all(|x| *x > 0) || arr.iter().all(|x| *x < 0));
}

fn part_one(input: &String) -> usize {
    let result = input
        .split("\n")
        .filter(|x| {
            let arr: Vec<i32> = x
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            is_safe(get_diffs(arr))
        })
        .collect::<Vec<&str>>()
        .len();

    result
}

fn part_two(input: &String) -> usize {
    let result = input
        .split("\n")
        .filter(|x| {
            let arr: Vec<i32> = x
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            let mut mutations = vec![arr.clone()];

            for i in 0..arr.len() {
                let mut arr_cloned = arr.clone();
                arr_cloned.remove(i);
                mutations.push(arr_cloned);
            }

            for mutation in mutations {
                if is_safe(get_diffs(mutation)) {
                    return true;
                }
            }

            return false;
        })
        .collect::<Vec<&str>>()
        .len();

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
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(&INPUT.to_string()), 2);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(&INPUT.to_string()), 4);
    }
}
