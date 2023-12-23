use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

advent_of_code::solution!(23);

#[derive(Debug, Clone)]
struct State {
    priority: u32,
    loc: (isize, isize),
    dir: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.dir == other.dir && self.visited == other.visited
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((y as isize, x as isize), c))
                .collect::<Vec<((isize, isize), char)>>()
        })
        .collect();
    let x_max = grid.keys().map(|k| k.1).max()?;
    let y_max = grid.keys().map(|k| k.0).max()?;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for x in 0..=x_max {
        if grid.get(&(0, x))? == &'.' {
            start = (0, x);
        }
        if grid.get(&(y_max, x))? == &'.' {
            end = (y_max, x);
        }
    }

    let mut distances: HashMap<State, u32> = HashMap::new();
    let mut q: BinaryHeap<State> = BinaryHeap::new();

    let s = State {
        loc: start,
        dir: (0, 1),
        priority: 0,
        visited: HashSet::new(),
    };

    q.push(s.clone());
    distances.insert(s.clone(), 0);

    while let Some(state) = q.pop() {
        if let Some(neighbs) = state.get_neighbors(&grid, (y_max, x_max), false) {
            for next_state in &neighbs {
                if next_state.priority > *distances.get(next_state).unwrap_or(&0) {
                    distances.insert(next_state.clone(), next_state.priority);
                    q.push(next_state.clone());
                }
            }
        }
    }

    distances
        .iter()
        .filter(|(key, _value)| key.loc == end)
        .map(|(_key, value)| value)
        .max()
        .copied()
}
impl State {
    fn get_neighbors(
        &self,
        grid: &HashMap<(isize, isize), char>,
        grid_dim: (isize, isize),
        ignore_slopes: bool,
    ) -> Option<Vec<State>> {
        let mut neighbors: Vec<State> = Vec::new();

        let (y, x) = self.loc;

        let mut new_dirs = Vec::new();
        match grid.get(&self.loc)? {
            '#' => return None,
            'v' => new_dirs.push((1, 0)),
            '>' => new_dirs.push((0, 1)),
            '<' => new_dirs.push((0, -1)),
            '^' => new_dirs.push((-1, 0)),
            '.' => new_dirs = Vec::from([(1, 0), (-1, 0), (0, 1), (0, -1)]),
            c => panic!("not expecting {}", c),
        }
        if ignore_slopes {
            new_dirs = Vec::from([(1, 0), (-1, 0), (0, 1), (0, -1)]);
        }
        for (dy, dx) in new_dirs {
            let (new_y, new_x) = (y + dy, x + dx);

            if new_y >= 0
                && new_y <= grid_dim.0
                && new_x >= 0
                && new_x <= grid_dim.1
                && !self.visited.contains(&(new_y, new_x))
                && grid.get(&(new_y, new_x))? != &'#'
            {
                let mut visited = self.visited.clone();
                visited.insert(self.loc);

                neighbors.push(State {
                    loc: (new_y, new_x),
                    dir: (dy, dx),
                    priority: self.priority + 1,
                    visited,
                });
            }
        }

        Some(neighbors)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((y as isize, x as isize), c))
                .collect::<Vec<((isize, isize), char)>>()
        })
        .collect();
    let x_max = grid.keys().map(|k| k.1).max()?;
    let y_max = grid.keys().map(|k| k.0).max()?;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for x in 0..=x_max {
        if grid.get(&(0, x))? == &'.' {
            start = (0, x);
        }
        if grid.get(&(y_max, x))? == &'.' {
            end = (y_max, x);
        }
    }

    let mut distances: HashMap<State, u32> = HashMap::new();
    // let mut q: BinaryHeap<State> = BinaryHeap::new();
    let mut q: Vec<State> = Vec::new();
    let s = State {
        loc: start,
        dir: (0, 1),
        priority: 0,
        visited: HashSet::new(),
    };

    q.push(s.clone());
    distances.insert(s.clone(), 0);

    while let Some(state) = q.pop() {
        // if state.loc == end {
        // return Some(state.priority);
        // }
        if let Some(neighbs) = state.get_neighbors(&grid, (y_max, x_max), true) {
            for next_state in &neighbs {
                if next_state.priority > *distances.get(next_state).unwrap_or(&0) {
                    distances.insert(next_state.clone(), next_state.priority);
                    q.push(next_state.clone());
                }
            }
        }
    }

    distances
        .iter()
        .filter(|(key, _value)| key.loc == end)
        .map(|(_key, value)| value)
        .max()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
