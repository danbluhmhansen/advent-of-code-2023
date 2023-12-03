use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.find('\n').unwrap();
    let input = &input.replace('\n', "");

    let regex_num = Regex::new(r"\d+").unwrap();
    let regex_sym = Regex::new(r"[^0-9.]").unwrap();

    let numbers = regex_num.find_iter(input);
    let symbols = regex_sym
        .find_iter(input)
        .map(|sym| sym.start())
        .collect::<Vec<_>>();

    Some(
        numbers
            .filter_map(|num| {
                let start = num.start();
                let end = num.end();

                let mut indices = vec![];

                if start >= line_len {
                    let start = start - line_len;
                    let end = end - line_len;

                    if start > 0 {
                        indices.push(start - 1);
                    }

                    for i in start..=end {
                        indices.push(i);
                    }
                }

                if start > 0 {
                    indices.push(start - 1);
                }

                if end <= input.len() {
                    indices.push(end);
                }

                if end <= input.len() - line_len {
                    let start = start + line_len;
                    let end = end + line_len;

                    if start % line_len != 0 {
                        indices.push(start - 1);
                    }

                    for i in start..end {
                        indices.push(i);
                    }

                    if end <= input.len() {
                        indices.push(end);
                    }
                }

                if indices.iter().any(|i| symbols.contains(i)) {
                    num.as_str().parse::<u32>().ok()
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_len = input.find('\n').unwrap();
    let input = &input.replace('\n', "");

    let regex_num = Regex::new(r"\d+").unwrap();
    let regex_sym = Regex::new(r"\*").unwrap();

    Some(
        regex_sym
            .find_iter(input)
            .map(|sym| sym.start())
            .filter_map(|sym| {
                let mut indices = vec![];

                if sym >= line_len {
                    let sym = sym - line_len;

                    if sym > 0 {
                        indices.push(sym - 1);
                    }

                    indices.push(sym);
                    indices.push(sym + 1);
                }

                if sym > 0 {
                    indices.push(sym - 1);
                }

                if sym < input.len() {
                    indices.push(sym + 1);
                }

                if sym <= input.len() - line_len {
                    let sym = sym + line_len;

                    if sym % line_len != 0 {
                        indices.push(sym - 1);
                    }

                    indices.push(sym);

                    if sym < input.len() {
                        indices.push(sym + 1);
                    }
                }

                if regex_num
                    .find_iter(input)
                    .filter(|num| indices.iter().any(|i| num.range().contains(i)))
                    .count()
                    == 2
                {
                    Some(
                        regex_num
                            .find_iter(input)
                            .filter(|num| indices.iter().any(|i| num.range().contains(i)))
                            .filter_map(|num| num.as_str().parse::<u32>().ok())
                            .fold(1, |acc, num| acc * num),
                    )
                } else {
                    None
                }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
