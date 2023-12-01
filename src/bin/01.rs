advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = line.chars().find(|c| c.is_numeric());
                let last = line.chars().rev().find(|c| c.is_numeric());
                let num = match (first, last) {
                    (Some(f), Some(l)) => f.to_string() + &l.to_string(),
                    (Some(f), _) => f.to_string(),
                    (_, Some(l)) => l.to_string(),
                    (_, _) => panic!(),
                };
                num.parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_vec = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut new_string = input.to_string();
    for (search, r_string) in num_vec.iter() {
        new_string = new_string.replace(search, r_string);
    }
    print!("new string: {}", new_string);
    part_one(&new_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
