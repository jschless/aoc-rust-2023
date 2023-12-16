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
    fn get_next(
        &self,
        cur_loc: (usize, usize),
        dimensions: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            LightDir::N => {
                if cur_loc.0 == 0 {
                    None
                } else {
                    Some((cur_loc.0 - 1, cur_loc.1))
                }
            }
            LightDir::E => {
                if cur_loc.1 == dimensions.1 {
                    None
                } else {
                    Some((cur_loc.0, cur_loc.1 + 1))
                }
            }

            LightDir::S => {
                if cur_loc.0 == dimensions.0 {
                    None
                } else {
                    Some((cur_loc.0 + 1, cur_loc.1))
                }
            }
            LightDir::W => {
                if cur_loc.1 == 0 {
                    None
                } else {
                    Some((cur_loc.0, cur_loc.1 - 1))
                }
            }
        }
    }

    fn to_int(self) -> i8 {
        match self {
            LightDir::N => 0,
            LightDir::E => 1,
            LightDir::S => 2,
            LightDir::W => 3,
        }
    }

    fn from_int(i: i8) -> Self {
        match i % 4 {
            0 => LightDir::N,
            1 => LightDir::E,
            2 => LightDir::S,
            3 | -1 => LightDir::W,
            _ => panic!("Got int {}", i),
        }
    }

    fn transition_lens(&self, c: char) -> (Self, bool) {
        match c {
            '/' => {
                match self {
                    // E -> N, N -> E, S -> W, W->s
                    LightDir::N | LightDir::S => {
                        return (LightDir::from_int(self.to_int() + 1), false);
                    }
                    LightDir::E | LightDir::W => {
                        return (LightDir::from_int(self.to_int() - 1), false);
                    }
                }
            }
            '\\' => {
                match self {
                    // E -> S, S -> E, N -> W, W->N
                    LightDir::N | LightDir::S => {
                        return (LightDir::from_int(self.to_int() - 1), false);
                    }
                    LightDir::E | LightDir::W => {
                        return (LightDir::from_int(self.to_int() + 1), false);
                    }
                }
            }
            '-' => match self {
                LightDir::N | LightDir::S => {
                    return (LightDir::E, true);
                }
                _ => {}
            },
            '|' => match self {
                LightDir::E | LightDir::W => {
                    return (LightDir::N, true);
                }
                _ => {}
            },
            _ => {}
        }
        (*self, false)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let dimensions = (grid.len() - 1, grid[0].len() - 1);
    let mut visited: HashSet<((usize, usize), LightDir)> = HashSet::new();

    let mut cur_lights: VecDeque<((usize, usize), LightDir)> = VecDeque::new();

    let (next_dir, _) = LightDir::E.transition_lens(grid[0][0]);
    cur_lights.push_back(((0, 0), next_dir));

    while let Some((loc, dir)) = cur_lights.pop_front() {
        visited.insert((loc, dir));
        if let Some(next_loc) = dir.get_next(loc, dimensions) {
            // if this doesn't take us off the map
            let (next_dir, split) = dir.transition_lens(grid[next_loc.0][next_loc.1]);
            if !visited.contains(&(next_loc, next_dir)) {
                cur_lights.push_back((next_loc, next_dir));
                if split
                    && !visited.contains(&(next_loc, LightDir::from_int(next_dir.to_int() + 2)))
                {
                    cur_lights.push_back((next_loc, LightDir::from_int(next_dir.to_int() + 2)));
                }
            }
        }
    }

    Some(
        visited
            .iter()
            .map(|(loc, _)| loc)
            .cloned()
            .collect::<HashSet<(usize, usize)>>()
            .len() as u32,
    )
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
