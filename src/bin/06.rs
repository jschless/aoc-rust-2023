advent_of_code::solution!(6);

fn parse_str(input: &str) -> Vec<(u64, u64)> {
    let (time_str, dist_str) = input.split_once('\n').unwrap();
    time_str
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(
            dist_str
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap()),
        )
        .collect::<Vec<(u64, u64)>>()
}

fn solve_quadratic(b: f64, c: f64) -> Option<(u32, u32)> {
    let sqrt_disc = (b * b - 4.0 * c).sqrt();

    let root1: u32 = ((-b + sqrt_disc) / 2.0).ceil() as u32;
    let root2: u32 = ((-b - sqrt_disc) / 2.0).floor() as u32;

    Some((root1, root2))
}

fn count_ways_to_win(time: u64, dist: u64) -> u32 {
    // let mut ways_to_win: u32 = 0;
    // the intercepts with current distance is algebraically determinable bc quadratic
    // distance = (time-wait_time)*wait_time
    // distance = time*wait_time - wait_time^2
    // x^2 - time*x + distance = 0
    let (upper, lower) = solve_quadratic(-(time as f64), dist as f64).unwrap();
    upper - lower - 1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_str(input)
            .iter()
            .map(|(t, d)| count_ways_to_win(*t, *d))
            .product::<u32>(),
    )
}

fn remove_whitespace_and_parse(input: &str) -> u64 {
    input
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time_str, dist_str) = input.split_once('\n').unwrap();
    let time = remove_whitespace_and_parse(time_str);
    let dist = remove_whitespace_and_parse(dist_str);
    Some(count_ways_to_win(time, dist))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
