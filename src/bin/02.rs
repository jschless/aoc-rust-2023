use std::cmp;

advent_of_code::solution!(2);

fn get_game_maxes(input: &str) -> Vec<(u32, u32, u32)> {
    // turn the input into a list of 3-tuple u32s, which corresponds to (max red, max blue, max green) for a given game
    input
        .lines()
        .map(|line| {
            let colon = line.find(':').unwrap(); // location of start of game, use enum to track game number
            let rel_str = &line[colon + 2..];
            rel_str
                .split("; ")
                .map(|draw| {
                    draw.split(", ")
                        .map(|cubes| match cubes.split_once(" ") {
                            Some((count, "red")) => (count.parse::<u32>().unwrap(), 0, 0),
                            Some((count, "green")) => (0, count.parse::<u32>().unwrap(), 0),
                            Some((count, "blue")) => (0, 0, count.parse::<u32>().unwrap()),
                            _ => (0, 0, 0),
                        }) // parse each cube and count into a 3-tuple corresponding to (r,g,b)
                        .fold((0, 0, 0), |acc, val| {
                            (acc.0 + val.0, acc.1 + val.1, acc.2 + val.2)
                        }) // add all draws together to get a single hand
                })
                .fold((0, 0, 0), |acc, val| {
                    (
                        cmp::max(acc.0, val.0),
                        cmp::max(acc.1, val.1),
                        cmp::max(acc.2, val.2),
                    )
                }) // max all draws together to get the max per color
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    const N_CUBES: (u32, u32, u32) = (12, 13, 14);
    Some(
        get_game_maxes(input)
            .iter()
            .enumerate()
            .filter(|(_, game)| game.0 <= N_CUBES.0 && game.1 <= N_CUBES.1 && game.2 <= N_CUBES.2)
            .inspect(|x| println!("games that are possible: {:?}", x))
            .map(|(i, _)| i as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        get_game_maxes(input)
            .iter()
            .map(|game| game.0 * game.1 * game.2)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
