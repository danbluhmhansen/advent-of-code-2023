advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (game, rest) = l.split_once(": ").unwrap();

                // let game = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();

                let (winning, actual) = rest.split_once(" | ").unwrap();

                let winning = winning
                    .split(' ')
                    .filter_map(|num| num.parse::<u32>().ok())
                    .collect::<Vec<_>>();

                calc_points(
                    actual
                        .split(' ')
                        .filter_map(|num| num.parse::<u32>().ok())
                        .filter(|num| winning.contains(num))
                        .count(),
                )
            })
            .sum(),
    )
}

fn calc_points(count: usize) -> u32 {
    match count {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        5 => 16,
        6 => 32,
        7 => 64,
        8 => 128,
        9 => 256,
        10 => 512,
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
