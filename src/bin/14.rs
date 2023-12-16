use itertools::iproduct;
use std::collections::HashMap;
advent_of_code::solution!(14);

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_iter(&self, dimensions: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        match self {
            Direction::North | Direction::West => iproduct!(
                (0..dimensions.0).collect::<Vec<usize>>(),
                (0..dimensions.1).collect::<Vec<usize>>()
            ),
            Direction::East => iproduct!(
                (0..dimensions.0).collect::<Vec<usize>>(),
                (0..dimensions.1).rev().collect::<Vec<usize>>()
            ),
            Direction::South => iproduct!(
                (0..dimensions.0).rev().collect::<Vec<usize>>(),
                (0..dimensions.1).collect::<Vec<usize>>()
            ),
        }
    }

    fn move_anti_gravity(&self, ind: usize) -> usize {
        match self {
            Direction::North | Direction::West => ind + 1,
            _ => {
                if ind > 0 {
                    ind - 1
                } else {
                    ind
                }
            }
        }
    }

    fn get_bottom(&self, dimensions: (usize, usize)) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => dimensions.0 - 1,
            Direction::East => dimensions.1 - 1,
            Direction::West => 0,
        }
    }
}

fn tilt(
    grid: &mut HashMap<(usize, usize), char>,
    direction: Direction,
    dimensions: (usize, usize),
) {
    let mut next_loc_map: HashMap<usize, usize> = HashMap::new();
    let bottom_ind = direction.get_bottom(dimensions);
    for (row, col) in direction.get_iter(dimensions) {
        match grid.get(&(row, col)).unwrap() {
            '.' => {}
            '#' => {
                if direction == Direction::West || direction == Direction::East {
                    next_loc_map.insert(row, direction.move_anti_gravity(col));
                } else {
                    next_loc_map.insert(col, direction.move_anti_gravity(row));
                }
            }
            'O' => {
                if direction == Direction::West || direction == Direction::East {
                    let new_col = next_loc_map.get(&row).unwrap_or(&bottom_ind);

                    if col != *new_col {
                        grid.insert((row, *new_col), 'O');
                        grid.insert((row, col), '.');
                    }
                    next_loc_map.insert(row, direction.move_anti_gravity(*new_col));
                } else {
                    let new_row = next_loc_map.get(&col).unwrap_or(&bottom_ind);

                    if row != *new_row {
                        grid.insert((*new_row, col), 'O');
                        grid.insert((row, col), '.');
                    }
                    next_loc_map.insert(col, direction.move_anti_gravity(*new_row));
                }
            }
            _ => {
                panic!("wrong char in grid");
            }
        }
    }
}

fn compute_north_load(grid: &HashMap<(usize, usize), char>, dimensions: (usize, usize)) -> u32 {
    let mut acc = 0;
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            if grid.get(&(row, col)).unwrap() == &'O' {
                acc += dimensions.0 - row;
            }
        }
    }
    acc as u32
}

fn _disp_grid(grid: &HashMap<(usize, usize), char>, dimensions: (usize, usize)) {
    for x in 0..dimensions.0 {
        for y in 0..dimensions.1 {
            print!("{}", grid.get(&(x, y)).unwrap());
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid_map: HashMap<(usize, usize), char> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            grid_map.insert((row, col), grid[row][col]);
        }
    }
    let dimensions = (grid.len(), grid[0].len());
    tilt(&mut grid_map, Direction::North, dimensions);
    // disp_grid(&grid_map, dimensions);
    Some(compute_north_load(&grid_map, dimensions))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid_map: HashMap<(usize, usize), char> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            grid_map.insert((row, col), grid[row][col]);
        }
    }

    let dimensions = (grid.len(), grid[0].len());

    let mut loads: Vec<u32> = Vec::new();
    const TOTAL_CYCLES: usize = 1000000000;
    let n_cycles = 500;
    let warm_up = 150;
    for _ in 0..n_cycles {
        for dir in vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            tilt(&mut grid_map, dir, dimensions);
        }
        loads.push(compute_north_load(&grid_map, dimensions));
    }

    let max_val = loads.iter().skip(warm_up).max().unwrap();

    let _cycle_len = loads
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, b)| b == max_val)
        .map(|(a, _)| a)
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|arr| arr[1] - arr[0])
        .next()
        .unwrap();

    let cycle_len = 7;
    loads
        .iter()
        .skip(warm_up)
        .nth((TOTAL_CYCLES - warm_up - 1) % cycle_len)
        .copied()
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
