use std::collections::HashMap;

advent_of_code::solution!(4);

type Coord = (i32, i32);
type Grid = HashMap<Coord, char>;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let start_positions = get_start_positions(&grid, 'X');

    let directions = vec![
        vec![[1, 0], [2, 0], [3, 0]],
        vec![[-1, 0], [-2, 0], [-3, 0]],
        vec![[0, 1], [0, 2], [0, 3]],
        vec![[0, -1], [0, -2], [0, -3]],
        vec![[1, 1], [2, 2], [3, 3]],
        vec![[-1, 1], [-2, 2], [-3, 3]],
        vec![[1, -1], [2, -2], [3, -3]],
        vec![[-1, -1], [-2, -2], [-3, -3]],
    ];

    let mut count: u64 = 0;
    for start in start_positions {
        for dir in &directions {
            let contents: Vec<char> = get_surrounding_letters(&grid, &start, dir);
            if contents == ['M', 'A', 'S'] {
                count += 1
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let start_positions = get_start_positions(&grid, 'A');

    let directions = vec![vec![[-1, -1], [1, 1]], vec![[-1, 1], [1, -1]]];

    let mut count: u64 = 0;
    for start in start_positions {
        let mut letters: Vec<Vec<char>> = Vec::new();
        for dir in directions.iter() {
            let single_cross: Vec<char> = get_surrounding_letters(&grid, start, dir);
            letters.push(single_cross);
        }
        if letters
            .iter()
            .all(|item| item == &vec!['M', 'S'] || item == &vec!['S', 'M'])
        {
            count += 1
        }
    }
    Some(count)
}

fn get_surrounding_letters(grid: &Grid, start: &Coord, directions: &Vec<[i32; 2]>) -> Vec<char> {
    directions.iter()
        .map(|[d_row, d_col]| {
            let new_row = start.0 + d_row;
            let new_col = start.1 + d_col;
            *grid.get(&(new_row, new_col)).unwrap_or(&'_')
        })
        .collect()
}

fn get_start_positions(grid: &Grid, c: char) -> Vec<&Coord> {
    grid.iter()
        .filter(|(_, &val)| val == c)
        .map(|(coord, _)| coord)
        .collect()
}

fn parse_grid(input: &str) -> Grid {
    let mut grid: Grid = HashMap::new();

    for (rid, row) in input.trim().lines().enumerate() {
        for (cid, col) in row.trim().chars().enumerate() {
            grid.insert((cid as i32, rid as i32), col);
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
