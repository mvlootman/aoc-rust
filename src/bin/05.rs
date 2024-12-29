advent_of_code::solution!(5);

type Rule = (i32, i32);

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_order(input);

    let sum: i32 = updates
        .iter()
        .filter(|update| check_update(&update, &rules))
        .map(|update| get_middle_number(&update))
        .sum();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_order(input);

    let mut invalid_updates = updates
        .iter()
        .filter(|update| !check_update(&update, &rules))
        .collect::<Vec<_>>();

    let mut total = 0;
    for x in invalid_updates.iter_mut() {
        let mut a = x.clone();
        a.sort_by(|a, b| {
                    if is_valid(*a, *b, &rules) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });

        total += get_middle_number(&&a);
    }
    Some(total as u64)
}

fn get_middle_number(update: &&Vec<i32>) -> i32 {
    let middle_idx = update.len() / 2;
    *update.get(middle_idx).unwrap_or(&0)
}

fn is_valid(num: i32, after: i32, rules: &Vec<Rule>) -> bool {
    !rules.contains(&(after, num))
}

fn check_update(update: &Vec<i32>, rules: &Vec<Rule>) -> bool {
    // check each number with the numbers that follow
    let mut update_valid = true;
    for (idx, num) in update.iter().enumerate() {
        let valid = update[idx + 1..].iter().all(|&x| is_valid(*num, x, rules));
        if !valid {
            update_valid = false;
            break;
        }
    }
    update_valid
}

fn parse_order(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| line.split('|').collect::<Vec<_>>())
        .map(|w| (w[0].parse().unwrap(), w[1].parse().unwrap()))
        .collect::<Vec<_>>();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
