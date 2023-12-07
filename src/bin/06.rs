advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, dists) = input.split_once('\n').unwrap();

    Some(
        times
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .zip(dists.split_whitespace().filter_map(|s| s.parse().ok()))
            .map(|(time, dist)| {
                (0..=time)
                    .filter(|speed| (time - speed) * speed > dist)
                    .count() as u32
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, dist) = input.split_once('\n').unwrap();

    let time: u64 = time
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();

    let dist: u64 = dist
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .replace('\n', "")
        .parse()
        .unwrap();

    Some(
        (0..=time)
            .filter(|speed| (time - speed) * speed > dist)
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
