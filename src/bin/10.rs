advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Pipe {
    fn neighbors(&self, (y, x): (i32, i32)) -> ((i32, i32), (i32, i32)) {
        match *self {
            Pipe::Vertical => ((y + 1, x), (y - 1, x)),
            Pipe::Horizontal => ((y, x + 1), (y, x - 1)),
            Pipe::NE => ((y - 1, x), (y, x + 1)),
            Pipe::NW => ((y - 1, x), (y, x - 1)),
            Pipe::SW => ((y + 1, x), (y, x - 1)),
            Pipe::SE => ((y + 1, x), (y, x + 1)),
            Pipe::Ground => ((y, x), (y, x)),
            Pipe::Start => unimplemented!(),
        }
    }

    fn from_char(c: char) -> Pipe {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Shouldn't have that char"),
        }
    }
}

fn get_val(grid: &[Vec<Pipe>], (y, x): (i32, i32)) -> Option<Pipe> {
    if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[0].len() as i32 {
        None
    } else {
        Some(grid[y as usize][x as usize])
    }
}

fn get_path(grid: &[Vec<Pipe>]) -> Option<Vec<(i32, i32)>> {
    let mut start = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Pipe::Start {
                start = (y as i32, x as i32);
                break;
            }
        }
    }

    let mut prev = start;
    let mut cur = (0, 0);
    let mut path: Vec<(i32, i32)> = vec![start];

    // now we have to find some neighbor
    for (y, x) in &[(0, 1), (1, 0)] {
        let new_loc = (start.0 + y, start.1 + x);
        let neighbors = get_val(grid, new_loc)?.neighbors(new_loc);
        if neighbors.0 == start || neighbors.1 == start {
            cur = new_loc;
            path.push(new_loc);
            break;
        }
    }

    loop {
        let neighbors = get_val(grid, cur)?.neighbors(cur);

        if neighbors.0 == prev {
            prev = cur;
            cur = neighbors.1;
        } else {
            prev = cur;
            cur = neighbors.0;
        }

        if cur == start {
            return Some(path);
        }
        path.push(cur);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Pipe>> = input
        .lines()
        .map(|line| line.chars().map(Pipe::from_char).collect())
        .collect();

    let path = get_path(&grid).unwrap();

    Some(path.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Pipe>> = input
        .lines()
        .map(|line| line.chars().map(Pipe::from_char).collect())
        .collect();

    let mut path = get_path(&grid).unwrap();

    path.push(path[0]);
    let area: i32 = path
        .windows(2)
        .map(|arr| arr[1].0 * arr[0].1 - arr[1].1 * arr[0].0)
        .sum::<i32>()
        / 2;

    Some((area.abs() - (path.len() as i32 - 1) / 2 + 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&fs::read_to_string("./data/examples/10_2.txt").unwrap());
        assert_eq!(result, Some(4));

        let result = part_two(&fs::read_to_string("./data/examples/10_3.txt").unwrap());
        assert_eq!(result, Some(8));

        let result = part_two(&fs::read_to_string("./data/examples/10_4.txt").unwrap());
        assert_eq!(result, Some(10));
    }
}
