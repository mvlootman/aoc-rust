advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (left, right) = get_sorted_list(&input.lines().collect());
    Some(sum_of_distance(left, right))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = get_sorted_list(&input.lines().collect());

    // map
    let res: i32 = left
        .iter()
        .map(|x| x * (right.iter().filter(|n| n == &x).count() as i32))
        .sum();

    Some(res as u64)
}

fn parse_line(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    (
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap(),
    )
}

fn get_sorted_list(lines: &Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let pairs: Vec<_> = lines.iter().map(|line| parse_line(&line)).collect();
    let mut left = pairs.iter().map(|x| x.0).collect::<Vec<_>>();
    let mut right = pairs.iter().map(|x| x.1).collect::<Vec<_>>();
    left.sort();
    right.sort();

    (left, right)
}

fn sum_of_distance(left: Vec<i32>, right: Vec<i32>) -> u64 {
    let sod: i32 = left
        .iter()
        .zip(right.iter())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| (x.0 - x.1).abs())
        .sum();

    sod as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
