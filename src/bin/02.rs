advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                l.split_once(": ").map(|(game, rest)| {
                    let sets = rest
                        .split("; ")
                        .map(|set| {
                            let blue = *set
                                .split(", ")
                                .filter_map(|s| {
                                    if s.contains("blue") {
                                        extract_num(s)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .first()
                                .unwrap_or(&0);

                            let green = *set
                                .split(", ")
                                .filter_map(|s| {
                                    if s.contains("green") {
                                        extract_num(s)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .first()
                                .unwrap_or(&0);

                            let red = *set
                                .split(", ")
                                .filter_map(|s| {
                                    if s.contains("red") {
                                        extract_num(s)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .first()
                                .unwrap_or(&0);

                            (blue, green, red)
                        })
                        .collect::<Vec<_>>();

                    game.split_once(' ')
                        .and_then(|(_, id)| id.parse::<u32>().ok())
                        .map(|game| (game, sets))
                })
            })
            .flatten()
            .filter_map(|(game, sets)| {
                if sets.into_iter().any(|(b, g, r)| b > 14 || g > 13 || r > 12) {
                    None
                } else {
                    Some(game)
                }
            })
            .sum::<u32>(),
    )
}

fn extract_num(s: &str) -> Option<u32> {
    s.split_once(' ').and_then(|(n, _)| n.parse::<u32>().ok())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                l.split_once(": ").map(|(_, rest)| {
                    let sets = rest.split("; ").map(|set| {
                        let blue = *set
                            .split(", ")
                            .filter_map(|s| {
                                if s.contains("blue") {
                                    extract_num(s)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .first()
                            .unwrap_or(&0);

                        let green = *set
                            .split(", ")
                            .filter_map(|s| {
                                if s.contains("green") {
                                    extract_num(s)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .first()
                            .unwrap_or(&0);

                        let red = *set
                            .split(", ")
                            .filter_map(|s| {
                                if s.contains("red") {
                                    extract_num(s)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .first()
                            .unwrap_or(&0);

                        (blue, green, red)
                    });

                    let b = sets.clone().map(|(b, _, _)| b).max().unwrap();
                    let g = sets.clone().map(|(_, g, _)| g).max().unwrap();
                    let r = sets.clone().map(|(_, _, r)| r).max().unwrap();

                    b * g * r
                })
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
