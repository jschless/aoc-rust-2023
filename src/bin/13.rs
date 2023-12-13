use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

advent_of_code::solution!(13);

fn transpose_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect()
}

fn gen_check(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    hasher.finish()
}

pub fn get_pattern_vals(input: &str) -> u32 {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    let row_hashes: Vec<u64> = grid.iter().map(|x| gen_check(x)).collect();
    let mut acc = 0;

    for row_split in 1..grid.len() {
        if row_hashes[..row_split]
            .iter()
            .rev()
            .zip(row_hashes[row_split..].iter())
            .all(|(a, b)| a == b)
        {
            // println!("splitting up of row {}", row_split);
            acc += 100 * row_split;
        }
    }

    let col_hashes: Vec<u64> = transpose_grid(&grid).iter().map(|x| gen_check(x)).collect();
    for col_split in 1..grid[0].len() {
        if col_hashes[..col_split]
            .iter()
            .rev()
            .zip(col_hashes[col_split..].iter())
            .all(|(a, b)| a == b)
        {
            // println!("splitting left of col {}", col_split);
            acc += col_split;
        }
    }
    acc as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n\n").map(|pat| get_pattern_vals(pat)).sum())
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
