use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn get_neighbors(
    loc: (isize, isize),
    grid: &HashMap<(isize, isize), char>,
    grid_dim: (isize, isize),
    ignore_slopes: bool,
) -> Vec<(isize, isize)> {
    let mut neighbors: Vec<(isize, isize)> = Vec::new();

    let (y, x) = loc;

    let mut new_dirs = Vec::new();
    match grid.get(&loc).unwrap() {
        '#' => return Vec::new(),
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
            && grid.get(&(new_y, new_x)).unwrap() != &'#'
        {
            neighbors.push((new_y, new_x));
        }
    }

    neighbors
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

    let mut dps: HashSet<(isize, isize)> = grid
        .iter()
        .filter_map(|(loc, _c)| {
            if get_neighbors(*loc, &grid, (y_max, x_max), false).len() > 2 {
                Some(*loc)
            } else {
                None
            }
        })
        .collect();

    dps.insert(start);
    dps.insert(end);
    let mut distances: HashMap<(isize, isize), HashMap<(isize, isize), u32>> = HashMap::new();

    for pt in dps.clone() {
        let mut dist_map = HashMap::new();
        let mut stack = Vec::from([(pt, 0)]);
        let mut seen = HashSet::from([pt]);

        while let Some((loc, dist)) = stack.pop() {
            if dist != 0 && dps.contains(&loc) {
                dist_map.insert(loc, dist);
                continue;
            }

            for n in get_neighbors(loc, &grid, (y_max, x_max), false) {
                if !seen.contains(&n) {
                    stack.push((n, dist + 1));
                    seen.insert(n);
                }
            }
        }
        distances.insert(pt, dist_map);
    }

    let mut largest = 0;
    let mut stack = Vec::from([(start, 0_u32, HashSet::from([start]))]);

    while let Some((loc, dist, path)) = stack.pop() {
        if loc == end && dist > largest {
            largest = dist;
        }

        for (next, cost) in &distances[&loc] {
            let mut new_path = path.clone();
            if !path.contains(&next) {
                new_path.insert(*next);
                stack.push((*next, dist + cost, new_path));
            }
        }
    }

    Some(largest)
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

    let mut dps: HashSet<(isize, isize)> = grid
        .iter()
        .filter_map(|(loc, _c)| {
            if get_neighbors(*loc, &grid, (y_max, x_max), false).len() > 2 {
                Some(*loc)
            } else {
                None
            }
        })
        .collect();

    dps.insert(start);
    dps.insert(end);
    let mut distances: HashMap<(isize, isize), HashMap<(isize, isize), u32>> = HashMap::new();

    for pt in dps.clone() {
        let mut dist_map = HashMap::new();
        let mut stack = Vec::from([(pt, 0)]);
        let mut seen = HashSet::from([pt]);

        while let Some((loc, dist)) = stack.pop() {
            if dist != 0 && dps.contains(&loc) {
                dist_map.insert(loc, dist);
                continue;
            }

            for n in get_neighbors(loc, &grid, (y_max, x_max), true) {
                if !seen.contains(&n) {
                    stack.push((n, dist + 1));
                    seen.insert(n);
                }
            }
        }
        distances.insert(pt, dist_map);
    }

    let mut largest = 0;
    let mut stack = Vec::from([(start, 0_u32, HashSet::from([start]))]);

    while let Some((loc, dist, path)) = stack.pop() {
        if loc == end && dist > largest {
            largest = dist;
        }

        for (next, cost) in &distances[&loc] {
            let mut new_path = path.clone();
            if !path.contains(&next) {
                new_path.insert(*next);
                stack.push((*next, dist + cost, new_path));
            }
        }
    }

    Some(largest)
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
