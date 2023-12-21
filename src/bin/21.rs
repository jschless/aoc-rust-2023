use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let grid_map: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(|(y, c)| ((x as isize, y as isize), c))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut start_loc = (0, 0);

    for (key, val) in &grid_map {
        if val == &'S' {
            start_loc = *key;
        }
    }

    let s = explore_dec(64, start_loc, &grid_map);
    Some(s.len() as u32)
}

fn explore_dec(
    steps: usize,
    start: (isize, isize),
    grid: &HashMap<(isize, isize), char>,
) -> HashSet<(isize, isize)> {
    let mut q: HashSet<(isize, isize)> = HashSet::new();
    q.insert(start);
    for _ in 0..steps {
        let mut next_q: HashSet<(isize, isize)> = HashSet::new();
        for last in q {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = (last.0 + dx, last.1 + dy);
                if grid.contains_key(&next) && next != last && grid[&next] != '#' {
                    next_q.insert(next);
                }
            }
        }
        q = next_q.clone();
    }
    q
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
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
