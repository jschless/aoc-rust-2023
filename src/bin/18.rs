use std::collections::HashSet;

advent_of_code::solution!(18);

fn _print_grid(path: &[(i32, i32)]) {
    let max_y = path.iter().map(|(x, _)| x).max().unwrap();
    let max_x = path.iter().map(|(_, y)| y).max().unwrap();
    let path_set: HashSet<(i32, i32)> = path.iter().cloned().collect();
    for i in 0..=*max_y {
        for j in 0..=*max_x {
            if path_set.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn compute_area(instr: &[((i64, i64), i64)]) -> Option<i64> {
    let mut path: Vec<(i64, i64)> = vec![(0, 0)];

    for (dir, mag) in instr {
        let last = path.iter().last().unwrap().to_owned();
        path.push((last.0 + dir.0 * mag, last.1 + dir.1 * mag));
    }

    let perim: i64 = instr.iter().map(|(_, mag)| mag).sum();

    let area = path
        .windows(2)
        .map(|arr| (arr[1].0 * arr[0].1 - arr[1].1 * arr[0].0))
        .sum::<i64>()
        .abs()
        / 2;

    Some(area + perim / 2 + 1)
}

pub fn part_one(input: &str) -> Option<i64> {
    let instr: Vec<((i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let temp: Vec<&str> = line.split_whitespace().collect();
            let dir: (i64, i64) = match temp[0] {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (-1, 0),
                "D" => (1, 0),
                _ => panic!("did not expect {}", temp[0]),
            };
            (dir, temp[1].parse::<i64>().unwrap())
        })
        .collect();

    compute_area(&instr)
}

pub fn part_two(input: &str) -> Option<i64> {
    let instr: Vec<((i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let hex_str: &str = line.split_whitespace().last().unwrap();
            let hex_str = &hex_str[1..hex_str.len() - 1];
            let mag = i64::from_str_radix(&hex_str[1..hex_str.len() - 1], 16).unwrap();
            let dir: (i64, i64) = match &hex_str[hex_str.len() - 1..] {
                "0" => (0, 1),
                "2" => (0, -1),
                "3" => (-1, 0),
                "1" => (1, 0),
                e => panic!("did not expect {}", e),
            };
            (dir, mag)
        })
        .collect();

    compute_area(&instr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
