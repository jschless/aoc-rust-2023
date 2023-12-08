use num_integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (Vec<usize>, HashMap<&str, Vec<&str>>) {
    let (pattern, maps) = input.split_once("\n\n").unwrap();
    let pattern_vec: Vec<usize> = pattern
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    let mut map_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in maps.lines() {
        let (key, vals) = line.split_once(" = (").unwrap();
        map_map.insert(
            key,
            vals.trim_end_matches(')')
                .split(", ")
                .collect::<Vec<&str>>(),
        );
    }
    (pattern_vec, map_map)
}

fn gen_solve<F>(
    start_key: &str,
    pattern: Vec<usize>,
    maps: HashMap<&str, Vec<&str>>,
    check_done: F,
) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut iters: u64 = 0;
    let mut ind: usize = 0;
    let mut cur_key = start_key;
    loop {
        iters += 1;
        let inst = *pattern.get(ind).unwrap();
        let cur_val = maps.get(cur_key).unwrap();
        cur_key = cur_val.get(inst).unwrap();

        if check_done(cur_key) {
            return iters;
        }
        ind = (ind + 1) % pattern.len(); // circular indexing
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pattern, maps) = parse_input(input);
    Some(gen_solve("AAA", pattern, maps, |s| s == "ZZZ"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (pattern, maps) = parse_input(input);
    let cur_keys: Vec<&str> = maps.keys().cloned().filter(|s| s.ends_with('A')).collect();
    cur_keys
        .iter()
        .map(|key| gen_solve(key, pattern.clone(), maps.clone(), |s| s.ends_with('Z')))
        .reduce(lcm)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&fs::read_to_string("./data/examples/08_2.txt").unwrap());
        assert_eq!(result, Some(6));
    }
}
