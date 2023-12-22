use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

#[derive(Debug, Clone)]
struct Brick {
    start: Point,
    end: Point,
    id: usize,
}

impl Brick {
    fn lowest_point(&self) -> isize {
        std::cmp::min(self.start.z, self.end.z)
    }

    fn on_ground(&self) -> bool {
        self.lowest_point() == 1
    }

    fn fall(&self) -> Self {
        Self {
            start: self.start.fall(),
            end: self.end.fall(),
            id: self.id,
        }
    }

    fn on_segment(p: Point, q: Point, r: Point) -> bool {
        (q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x)) && (q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y))
    }

    fn orientation(p: Point, q: Point, r: Point) -> i32 {
        let val = ((q.y - p.y) * (r.x - q.x)) - ((q.x - p.x) * (r.y - q.y));

        if val == 0 {
            return 0; // colinear
        }

        if val > 0 {
            return 1; // clockwise
        }

        2 // counterclockwise
    }

    fn do_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
        let o1 = Brick::orientation(p1, q1, p2);
        let o2 = Brick::orientation(p1, q1, q2);
        let o3 = Brick::orientation(p2, q2, p1);
        let o4 = Brick::orientation(p2, q2, q1);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == 0 && Brick::on_segment(p1, p2, q1) {
            return true;
        }

        if o2 == 0 && Brick::on_segment(p1, q2, q1) {
            return true;
        }

        if o3 == 0 && Brick::on_segment(p2, p1, q2) {
            return true;
        }

        if o4 == 0 && Brick::on_segment(p2, q1, q2) {
            return true;
        }

        false
    }

    fn is_touching(&self, b: Brick) -> bool {
        if std::cmp::min(self.start.z, self.end.z) >= std::cmp::min(b.start.z, b.end.z)
            && std::cmp::min(self.start.z, self.end.z) <= std::cmp::max(b.start.z, b.end.z)
        {
            Brick::do_intersect(self.start, self.end, b.start, b.end)
        } else {
            false
        }
    }
}
#[derive(Debug, Clone, Copy)]
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

    fn fall(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
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

    bricks.sort_by_key(|brick| brick.lowest_point());

    let mut new_bricks: Vec<Brick> = Vec::new();
    let mut brick_supporters: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut brick_supported: HashMap<usize, Vec<usize>> = HashMap::new();

    for b in bricks {
        brick_supporters.insert(b.id, Vec::new());
        brick_supported.insert(b.id, Vec::new());
        let mut poss_brick = b.clone();
        loop {
            let mut flag = false;
            for existing_brick in new_bricks.clone() {
                if poss_brick.fall().is_touching(existing_brick.clone()) {
                    flag = true;
                    brick_supporters
                        .get_mut(&existing_brick.id)?
                        .push(poss_brick.id);

                    brick_supported
                        .get_mut(&poss_brick.id)?
                        .push(existing_brick.id);
                }
            }

            if flag || poss_brick.on_ground() {
                new_bricks.push(poss_brick.clone());
                break;
            }

            poss_brick = poss_brick.fall();
        }
    }
    let mut count = 0;
    let mut bricks: Vec<_> = Vec::new();
    for (b, supported) in brick_supporters.clone() {
        // for each supporting brick B1, check for all the bricks Bn its supporting that # supporters(Bn) != 1
        if supported.iter().all(|s| brick_supported[s].len() != 1) {
            count += 1;
            bricks.push(b);
        }
    }
    Some(count)
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
