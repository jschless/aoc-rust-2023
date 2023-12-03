use std::collections::HashMap;

advent_of_code::solution!(3);

fn vec_to_num(v: Vec<char>) -> u32 {
    // converts a vector of numeric chars to a u32
    v.iter().rev().enumerate().fold(0, |acc, v| {
        acc + 10_u32.pow(v.0 as u32) * v.1.to_digit(10).unwrap()
    })
}

fn conform_to_bounds(i: i32, u_bound: i32) -> usize {
    // corrects neighbor checking where we go off the map
    std::cmp::min(std::cmp::max(0, i), u_bound) as usize
}

fn construct_sym_map(input: &str) -> HashMap<(usize, usize), Vec<u32>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    const NEIGHBORS: [(i32, i32); 9] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut near_sym: Option<(usize, usize)> = None; // keeps track of whether the current number is near a symbol
    let mut num_vec: Vec<char> = Vec::new(); // keeps track of the current number
    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].is_numeric() {
                num_vec.push(grid[i][j]);

                // check neighbors to see if a symbol is contained if there is no symbol
                if near_sym.is_none() {
                    for (x, y) in NEIGHBORS {
                        let new_x = conform_to_bounds(i as i32 + x, grid.len() as i32 - 1);
                        let new_y = conform_to_bounds(j as i32 + y, grid[i].len() as i32 - 1);

                        match grid[new_x][new_y] {
                            '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
                            _ => {
                                near_sym = Some((new_x, new_y));
                                break;
                            }
                        };
                    }
                }
            }
            /* if it's not a number OR its the end of the row
            THEN check if we touched a symbol
            If we did, add the number that was collected to the hashmap
            Reset our variables
            */
            if j == grid[i].len() - 1 || !grid[i][j].is_numeric() {
                if let Some(sym_loc) = near_sym {
                    map.entry(sym_loc).or_insert(Vec::new());
                    if let Some(ve) = map.get_mut(&sym_loc) {
                        ve.push(vec_to_num(num_vec));
                    }
                }
                num_vec = Vec::new();
                near_sym = None;
            }
        }
    }
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: HashMap<(usize, usize), Vec<u32>> = construct_sym_map(input);
    Some(map.values().map(|v| v.iter().sum::<u32>()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = construct_sym_map(input);
    Some(
        map.values()
            .filter(|v| v.len() > 1)
            .map(|v| v.iter().fold(1, |acc, x| x * acc))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
