advent_of_code::solution!(1);

// TODO: refactors before git, no longer works
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|s| {
                let digits = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
                if digits.len() > 0 {
                    Some(digits[0] * 10 + *digits.last().unwrap())
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let chars = l.chars();
                let collect = l.chars().collect::<String>();
                let digits = chars
                    .enumerate()
                    .filter_map(|(i, c)| match c {
                        '1' => Some(1),
                        '2' => Some(2),
                        '3' => Some(3),
                        '4' => Some(4),
                        '5' => Some(5),
                        '6' => Some(6),
                        '7' => Some(7),
                        '8' => Some(8),
                        '9' => Some(9),
                        'o' if collect.get(i..i + 3).is_some_and(|one| one == "one") => Some(1),
                        't' if collect.get(i..i + 3).is_some_and(|two| two == "two") => Some(2),
                        't' if collect.get(i..i + 5).is_some_and(|three| three == "three") => {
                            Some(3)
                        }
                        'f' if collect.get(i..i + 4).is_some_and(|four| four == "four") => Some(4),
                        'f' if collect.get(i..i + 4).is_some_and(|five| five == "five") => Some(5),
                        's' if collect.get(i..i + 3).is_some_and(|six| six == "six") => Some(6),
                        's' if collect.get(i..i + 5).is_some_and(|seven| seven == "seven") => {
                            Some(7)
                        }
                        'e' if collect.get(i..i + 5).is_some_and(|eight| eight == "eight") => {
                            Some(8)
                        }
                        'n' if collect.get(i..i + 4).is_some_and(|nine| nine == "nine") => Some(9),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                digits[0] * 10 + *digits.last().unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
