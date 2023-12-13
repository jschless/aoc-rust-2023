advent_of_code::solution!(13);

fn transpose_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect()
}

fn xor_sum(d1: &[u8], d2: &[u8]) -> u8 {
    // returns the sum of xor values for two vecs
    d1.iter().zip(d2.iter()).map(|(a, b)| a ^ b).sum()
}

fn find_changes(v: &[Vec<u8>], allowable_diffs: u8) -> u32 {
    (1..v.len())
        .map(|split| {
            if v.iter()
                .take(split)
                .rev()
                .zip(v.iter().skip(split))
                .map(|(a, b)| xor_sum(a, b))
                .sum::<u8>()
                == allowable_diffs
            // gets sum of xor differences
            {
                split
            } else {
                0
            }
        })
        .sum::<usize>() as u32
}

pub fn get_pattern_vals(input: &str, allowable_diff: u8) -> u32 {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    100 * find_changes(&grid, allowable_diff) + find_changes(&transpose_grid(&grid), allowable_diff)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n\n").map(|p| get_pattern_vals(p, 0)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("\n\n").map(|p| get_pattern_vals(p, 1)).sum())
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
        assert_eq!(result, Some(400));
    }
}
