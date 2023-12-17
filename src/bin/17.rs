use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone)]
struct State {
    loc: (usize, usize),
    dir: (i32, i32),
    n_cont: usize,
    priority: u32,
}

impl State {
    fn get_neighbors(&self, grid: &[Vec<u8>], move_bounds: (i32, i32)) -> Vec<State> {
        let mut neighbors: Vec<State> = Vec::new();
        let dim = (grid.len() as i32, grid[0].len() as i32);
        let (dy, dx) = self.dir;
        let y = self.loc.0 as i32;
        let x = self.loc.1 as i32;
        // turn
        for d in move_bounds.0..move_bounds.1 {
            for (i, j) in [(-dx, -dy), (dx, dy), (dy, dx)] {
                // dbg!(x, j);
                let new_x = x + j * d;
                let new_y = y + i * d;
                if new_y >= 0 && new_y < dim.0 && new_x >= 0 && new_x < dim.1 {
                    let n_cont = if self.dir == (i, j) {
                        self.n_cont + d as usize
                    } else {
                        d as usize
                    };
                    let mut cost = 0;
                    for step in 1..=d {
                        cost += grid[(y + i * step) as usize][(x + j * step) as usize];
                    }

                    let priority = self.priority + cost as u32;

                    if n_cont >= move_bounds.1 as usize {
                        continue;
                    }

                    neighbors.push(State {
                        loc: (new_y as usize, new_x as usize),
                        dir: (i, j),
                        n_cont,
                        priority,
                    });
                }
            }
        }

        neighbors
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.dir == other.dir && self.n_cont == other.n_cont
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.dir.hash(state);
        self.n_cont.hash(state);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

pub fn solver(input: &str, move_bounds: (i32, i32)) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut distances: HashMap<State, u32> = HashMap::new();
    let mut q: BinaryHeap<State> = BinaryHeap::new();

    // fill q with state space
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                for n_cont in move_bounds.0..move_bounds.1 {
                    let state = State {
                        loc: (i, j),
                        dir,
                        n_cont: n_cont as usize,
                        priority: u32::MAX - 1000,
                    };
                    distances.insert(state, state.priority);
                    q.push(state);
                }
            }
        }
    }

    for dir in [(0, 1), (1, 0)] {
        let s = State {
            loc: (0, 0),
            dir,
            n_cont: 0,
            priority: 0,
        };
        q.push(s);
        distances.insert(s, 0);
    }

    while !q.is_empty() {
        let state = q.pop().unwrap();

        for next_state in state.get_neighbors(&grid, move_bounds) {
            if next_state.priority < distances[&next_state] {
                distances.insert(next_state, next_state.priority);
                q.push(next_state);
            }
        }
    }

    distances
        .iter()
        .filter(|(key, _value)| key.loc == (grid.len() - 1, grid[0].len() - 1))
        .map(|(_key, value)| value)
        .min()
        .copied()
}

pub fn part_one(input: &str) -> Option<u32> {
    solver(input, (1, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    solver(input, (4, 11))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&fs::read_to_string("./data/examples/17_2.txt").unwrap());
        assert_eq!(result, Some(71));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
