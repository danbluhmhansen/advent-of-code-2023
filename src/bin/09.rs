advent_of_code::solution!(9);

fn calc_patterns_one(history: &Vec<i32>) -> i32 {
    if !history.iter().all(|n| *n == 0) {
        history.last().unwrap()
            + calc_patterns_one(&history.windows(2).map(|w| w[1] - w[0]).collect())
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .fold(0, |acc, history| acc + calc_patterns_one(&history)),
    )
}

fn calc_patterns_two(history: &Vec<i32>) -> i32 {
    if !history.iter().all(|n| *n == 0) {
        history.first().unwrap()
            - calc_patterns_two(&history.windows(2).map(|w| w[1] - w[0]).collect())
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .fold(0, |acc, history| acc + calc_patterns_two(&history)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
