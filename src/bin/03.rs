advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for matches in regex.captures_iter(input) {
        let x = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();

        total += x * y
    }

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut enabled_state = true;
    for matches in regex.captures_iter(input) {
        match matches.get(0).unwrap().as_str() {
            "do()" => enabled_state = true,
            "don't()" => enabled_state = false,
            _ => {
                if enabled_state {
                    let x = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    total += x * y
                }
            }
        }
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
