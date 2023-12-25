use std::collections::{HashMap, VecDeque};

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

    let visited = explore_dec(start_loc, &grid_map);
    Some(
        visited
            .values()
            .filter(|v| **v % 2 == 0 && **v <= 64)
            .count() as u32,
    )
}

fn explore_dec(
    start: (isize, isize),
    grid: &HashMap<(isize, isize), char>,
) -> HashMap<(isize, isize), usize> {
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();
    let mut to_explore: VecDeque<((isize, isize), usize)> = VecDeque::from([(start, 0)]);
    while let Some((last, dist)) = to_explore.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (last.0 + dx, last.1 + dy);
            if grid.contains_key(&next) && grid[&next] != '#' && !visited.contains_key(&next) {
                to_explore.push_back((next, dist + 1));
                visited.insert(next, dist + 1);
            }
        }
    }

    visited
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid_map: HashMap<(isize, isize), char> = input
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
    grid_map.insert(start_loc, '.');

    let grid_max = grid_map.keys().map(|k| k.1).max()? + 1;

    let visited = explore_dec(start_loc, &grid_map);
    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();

    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

    let n = ((26501365 - (grid_max / 2)) / grid_max) as usize;

    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;

    Some(p2 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
