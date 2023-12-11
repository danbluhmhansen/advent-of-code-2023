advent_of_code::solution!(11);

fn calc_dist(
    (ax, ay): &(usize, usize),
    (bx, by): &(usize, usize),
    dbl_rows: &Vec<usize>,
    dbl_cols: &Vec<usize>,
) -> u32 {
    let x = ax.min(bx)..=ax.max(bx);
    let y = ay.min(by)..=ay.max(by);
    let dbl_cols = dbl_cols.iter().filter(|i| x.contains(i)).count();
    let dbl_rows = dbl_rows.iter().filter(|i| y.contains(i)).count();
    ((**x.end() - **x.start()) + (**y.end() - **y.start()) + dbl_rows + dbl_cols)
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.find('\n').unwrap();

    let input = input.replace('\n', "");
    let len = input.len();

    let input = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then(|| (i % line_len, i / line_len)))
        .collect::<Vec<_>>();

    let dbl_rows = (0..(len / line_len))
        .filter(|i| !input.iter().any(|(_, y)| i == y))
        .collect::<Vec<_>>();

    let dbl_cols = (0..line_len)
        .filter(|i| !input.iter().any(|(x, _)| i == x))
        .collect::<Vec<_>>();

    let mut total = 0;
    for i in 0..input.len() {
        for e in &input[i + 1..] {
            total += calc_dist(input.get(i).unwrap(), e, &dbl_rows, &dbl_cols);
        }
    }

    Some(total)
}

fn calc_dist_two(
    (ax, ay): &(usize, usize),
    (bx, by): &(usize, usize),
    dbl_rows: &Vec<usize>,
    dbl_cols: &Vec<usize>,
) -> u64 {
    let x = ax.min(bx)..=ax.max(bx);
    let y = ay.min(by)..=ay.max(by);
    let dbl_cols = dbl_cols.iter().filter(|i| x.contains(i)).count();
    let dbl_rows = dbl_rows.iter().filter(|i| y.contains(i)).count();
    ((**x.end() - **x.start())
        + (**y.end() - **y.start())
        + (dbl_rows * 999999)
        + (dbl_cols * 999999))
        .try_into()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let line_len = input.find('\n').unwrap();

    let input = input.replace('\n', "");
    let len = input.len();

    let input = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then(|| (i % line_len, i / line_len)))
        .collect::<Vec<_>>();

    let dbl_rows = (0..(len / line_len))
        .filter(|i| !input.iter().any(|(_, y)| i == y))
        .collect::<Vec<_>>();

    let dbl_cols = (0..line_len)
        .filter(|i| !input.iter().any(|(x, _)| i == x))
        .collect::<Vec<_>>();

    let mut total = 0;
    for i in 0..input.len() {
        for e in &input[i + 1..] {
            total += calc_dist_two(input.get(i).unwrap(), e, &dbl_rows, &dbl_cols);
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
