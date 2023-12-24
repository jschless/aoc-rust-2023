advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl Coord {
    fn from_str(s: &str) -> Self {
        let temp: Vec<_> = s
            .split(',')
            .map(|i| i.trim().parse::<f64>().unwrap())
            .collect();
        Coord {
            x: temp[0],
            y: temp[1],
            z: temp[2],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hail {
    pos: Coord,
    vel: Coord,
}

impl Hail {
    fn intersect(&self, other: Hail) -> Option<Coord> {
        let t_num =
            self.pos.y - other.pos.y - other.vel.y * (self.pos.x - other.pos.x) / other.vel.x;
        let t_den = (other.vel.y * self.vel.x / other.vel.x) - self.vel.y;

        let t = t_num / t_den;
        let s = (self.vel.x * t + self.pos.x - other.pos.x) / other.vel.x;
        if t < 0.0 || s < 0.0 {
            None
        } else {
            Some(self.apply(t))
        }
    }

    fn apply(&self, t: f64) -> Coord {
        Coord {
            x: self.pos.x + self.vel.x * t,
            y: self.pos.y + self.vel.y * t,
            z: self.pos.z + self.vel.z * t,
        }
    }
}

fn count_intersections(hailstones: &[Hail], min: f64, max: f64) -> u32 {
    (0..hailstones.len())
        .flat_map(|i| {
            (i..hailstones.len())
                .filter_map(move |j| {
                    let pos = hailstones[i].intersect(hailstones[j])?;
                    Some(pos.x >= min && pos.x <= max && pos.y >= min && pos.y <= max)
                })
                .filter(|x| *x)
        })
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones: Vec<_> = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            Hail {
                pos: Coord::from_str(pos),
                vel: Coord::from_str(vel),
            }
        })
        .collect();

    // Some(count_intersections(&hailstones, 7.0, 27.0))
    Some(count_intersections(
        &hailstones,
        200000000000000.0,
        400000000000000.0,
    ))
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
