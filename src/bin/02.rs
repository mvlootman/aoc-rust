advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_lines = parse_input(input);
    let count = parsed_lines.iter().filter(is_safe).count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_lines = parse_input(input);

    let mut count = 0;
    for line in parsed_lines {
        for numbers in variants(&line) {
            if is_safe(&&numbers) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.parse().unwrap())
                .collect()
        })
        .collect()
}
fn is_safe(line: &&Vec<i32>) -> bool {
    let mut sort_asc = (*line).clone();
    sort_asc.sort();

    let mut sort_desc = sort_asc.clone();
    sort_desc.reverse();

    let within_bounds = line
        .windows(2)
        .map(|item| {
            let safe = if let [a, b] = item {
                let delta = (a - b).abs();
                delta > 0 && delta < 4
            } else {
                false
            };
            safe
        })
        .all(|x| x == true);

    within_bounds && (line == &&sort_desc || line == &&sort_asc)
}

fn variants(start: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut variants: Vec<Vec<i32>> = Vec::new();

    variants.push(start.clone());
    for (idx, _num) in start.iter().enumerate() {
        let mut variant = start.clone();
        variant.remove(idx);
        variants.push(variant);
    }
    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
