use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    end: Point,
    id: usize,
}

impl Brick {
    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for x in std::cmp::min(self.start.x, self.end.x)..=std::cmp::max(self.start.x, self.end.x) {
            for y in
                std::cmp::min(self.start.y, self.end.y)..=std::cmp::max(self.start.y, self.end.y)
            {
                for z in std::cmp::min(self.start.z, self.end.z)
                    ..=std::cmp::max(self.start.z, self.end.z)
                {
                    points.push(Point { x, y, z });
                }
            }
        }
        points
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let coords = s.split(',').collect::<Vec<_>>();
        let x = coords[0].parse::<isize>().unwrap();
        let y = coords[1].parse::<isize>().unwrap();
        let z = coords[2].parse::<isize>().unwrap();
        Point { x, y, z }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start, end) = line.split_once('~').unwrap();
            let start = Point::from_str(start);
            let end = Point::from_str(end);
            let id = i;
            Brick { start, end, id }
        })
        .collect();

    bricks.sort_by_key(|brick| std::cmp::min(brick.start.z, brick.end.z));

    let mut brick_supporters: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut brick_supported: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut z_map: HashMap<(isize, isize), isize> = HashMap::new();
    let mut grid: HashMap<Point, usize> = HashMap::new();

    for b in bricks {
        brick_supporters.insert(b.id, HashSet::new());
        brick_supported.insert(b.id, HashSet::new());
        let points = b.points();

        // find z at highest collision
        let new_z = points
            .iter()
            .map(|p| *z_map.entry((p.x, p.y)).or_insert(0))
            .max()?
            + 1;

        let b_bottom = std::cmp::min(b.start.z, b.end.z);

        // loop through all points for this brick
        for p in points {
            let possible_collision_point = Point {
                x: p.x,
                y: p.y,
                z: new_z - 1,
            };
            // if there is a brick at this grid, update the supporters/supportin
            if let Some(existing_brick_id) = grid.get(&possible_collision_point) {
                brick_supporters
                    .entry(*existing_brick_id)
                    .or_insert(HashSet::new())
                    .insert(b.id);
                brick_supported
                    .entry(b.id)
                    .or_insert(HashSet::new())
                    .insert(*existing_brick_id);
            }

            // update this position to be this grid
            z_map.insert((p.x, p.y), new_z + (p.z - b_bottom));
            grid.insert(
                Point {
                    x: p.x,
                    y: p.y,
                    z: new_z + (p.z - b_bottom),
                },
                b.id,
            );
        }
    }

    let mut count = 0;
    for (_b, supported) in brick_supporters.clone() {
        // for each supporting brick B1, check for all the bricks Bn its supporting that # supporters(Bn) != 1
        if supported.iter().all(|s| brick_supported[s].len() != 1) {
            count += 1;
        }
    }
    Some(count)
    // None
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
