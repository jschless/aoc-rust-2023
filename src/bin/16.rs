use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
enum LightDir {
    N,
    E,
    S,
    W,
}

impl LightDir {
    fn vel(&self) -> (i8, i8) {
        match self {
            LightDir::N => (-1, 0),
            LightDir::E => (0, 1),
            LightDir::S => (1, 0),
            LightDir::W => (0, -1),
        }
    }

    fn from_vel(&self, vel: (i8, i8)) -> Self {
        match vel {
            (-1, 0) => LightDir::N,
            (0, 1) => LightDir::E,
            (1, 0) => LightDir::S,
            (0, -1) => LightDir::W,
            _ => *self,
        }
    }

    fn transition_lens(&self, c: char) -> (Self, bool) {
        let vel = self.vel();
        let mut to_split = false;
        let new_dir = match c {
            '/' => self.from_vel((-vel.1, -vel.0)),
            '\\' => self.from_vel((vel.1, vel.0)),

            '-' => {
                let temp = self.from_vel((0, vel.0));
                if temp != *self {
                    to_split = true;
                }
                temp
            }
            '|' => {
                let temp = self.from_vel((vel.1, 0));
                if temp != *self {
                    to_split = true;
                }
                temp
            }
            _ => *self,
        };

        (new_dir, to_split)
    }

    fn get_next(
        &self,
        cur_loc: (usize, usize),
        dimensions: (usize, usize),
    ) -> Option<(usize, usize)> {
        let vel = self.vel();
        let next_loc = (cur_loc.0 as i8 + vel.0, cur_loc.1 as i8 + vel.1);
        if next_loc.0 < 0
            || next_loc.1 < 0
            || next_loc.0 > dimensions.0 as i8
            || next_loc.1 > dimensions.1 as i8
        {
            None
        } else {
            Some((next_loc.0 as usize, next_loc.1 as usize))
        }
    }
}

fn energize(grid: &[Vec<char>], start_state: ((usize, usize), LightDir)) -> u32 {
    let dimensions = (grid.len() - 1, grid[0].len() - 1);
    let mut visited: HashSet<((usize, usize), LightDir)> = HashSet::new();
    let mut cur_lights: VecDeque<((usize, usize), LightDir)> = VecDeque::new();
    cur_lights.push_front(start_state);

    while let Some((loc, dir)) = cur_lights.pop_front() {
        visited.insert((loc, dir));
        if let Some(next_loc) = dir.get_next(loc, dimensions) {
            let (next_dir, split) = dir.transition_lens(grid[next_loc.0][next_loc.1]);
            if !visited.contains(&(next_loc, next_dir)) {
                cur_lights.push_back((next_loc, next_dir));
            }
            if split {
                let vel = next_dir.vel();
                cur_lights.push_back((next_loc, next_dir.from_vel((-vel.0, -vel.1))));
            }
        }
    }

    visited
        .iter()
        .map(|(loc, _)| loc)
        .cloned()
        .collect::<HashSet<(usize, usize)>>()
        .len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let start_loc = (0, 0);
    let start_dir = LightDir::E
        .transition_lens(grid[start_loc.0][start_loc.1])
        .0;

    Some(energize(&grid, (start_loc, start_dir)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let mut start_vec: Vec<((usize, usize), LightDir)> = Vec::new();
    for col in 0..=max_col {
        start_vec.push(((0, col), LightDir::S));
        start_vec.push(((max_row, col), LightDir::N));
    }

    for row in 0..=max_row {
        start_vec.push(((row, 0), LightDir::E));
        start_vec.push(((row, max_col), LightDir::W));
    }
    start_vec
        .iter()
        .cloned()
        .map(|(start_pos, start_dir)| {
            energize(
                &grid,
                (
                    start_pos,
                    start_dir.transition_lens(grid[start_pos.0][start_pos.1]).0,
                ),
            )
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
