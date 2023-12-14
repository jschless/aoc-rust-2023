use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // IDEA: each rounded rock can fall where the next rounded rock or next cube rock + 1
    // keep hash map with next locs
    let mut map: HashMap<usize, usize> = HashMap::new();
    let n_rows = grid.len();
    let mut acc = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match grid[row][col] {
                '.' => {}
                '#' => {
                    map.insert(col, row + 1);
                }
                'O' => {
                    // rock can move to position map.get(col, 0)
                    let new_row = map.get(&col).unwrap_or(&0);
                    acc += n_rows - new_row;
                    map.insert(col, new_row + 1);
                }
                _ => {
                    panic!("wrong char in grid");
                }
            }
        }
    }
    Some(acc as u32)
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
