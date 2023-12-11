use itertools::{iproduct, Itertools};

advent_of_code::solution!(11);

fn transpose_grid(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect()
}

fn get_empty_rows(grid: &[Vec<bool>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .map(|(i, b)| if b.iter().all(|x| !*x) { i } else { 6969 })
        .filter(|x| *x != 6969)
        .collect()
}

fn get_galaxy_locs(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(i, j)| grid[i][j])
        .collect()
}

fn manhattan_plus(
    start: (usize, usize),
    end: (usize, usize),
    xs: &[usize],
    ys: &[usize],
    factor: u64,
) -> u64 {
    let min_y = std::cmp::min(start.0, end.0);
    let min_x = std::cmp::min(start.1, end.1);
    let max_y = std::cmp::max(start.0, end.0);
    let max_x = std::cmp::max(start.1, end.1);

    let n_traversed = xs
        .iter()
        .cloned()
        .filter(|x| x > &min_x && x < &max_x)
        .count()
        + ys.iter()
            .cloned()
            .filter(|y| y > &min_y && y < &max_y)
            .count();

    n_traversed as u64 * (factor - 1) + (max_x - min_x + max_y - min_y) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let grid_t = transpose_grid(&grid);

    let cols_to_add = get_empty_rows(&grid);

    let rows_to_add = get_empty_rows(&grid_t);

    Some(
        get_galaxy_locs(&grid)
            .iter()
            .combinations(2)
            .map(|v| manhattan_plus(*v[0], *v[1], &rows_to_add, &cols_to_add, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let grid_t = transpose_grid(&grid);

    let cols_to_add = get_empty_rows(&grid);

    let rows_to_add = get_empty_rows(&grid_t);

    Some(
        get_galaxy_locs(&grid)
            .iter()
            .combinations(2)
            .map(|v| manhattan_plus(*v[0], *v[1], &rows_to_add, &cols_to_add, 1000000))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
