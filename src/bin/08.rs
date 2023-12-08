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
    return (pattern_vec, map_map);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pattern, maps) = parse_input(input);
    let mut iters: u32 = 0;
    let mut ind: usize = 0;
    let mut cur_key = "AAA";
    loop {
        iters += 1;
        let inst = *pattern.get(ind).unwrap();
        // dbg!(inst, cur_key, &map_map);
        let cur_val = maps.get(cur_key).unwrap();
        cur_key = cur_val.get(inst).unwrap();

        if cur_key == "ZZZ" {
            return Some(iters);
        }
        ind = (ind + 1) % pattern.len(); // circular indexing
    }
}

fn vec_lcm(nums: Vec<u64>) -> u64 {
    let mut acc: u64 = *nums.first().unwrap();
    for n in nums.iter().skip(1).cloned() {
        acc = lcm(acc, n);
    }
    acc
}

pub fn part_two(input: &str) -> Option<u64> {
    let (pattern, maps) = parse_input(input);

    let cur_keys: Vec<&str> = maps.keys().cloned().filter(|s| s.ends_with('A')).collect();
    let ind_answers: Vec<u64> = cur_keys
        .iter()
        .map(|key| {
            let mut iters: u64 = 0;
            let mut ind: usize = 0;
            let mut cur_key = key;
            loop {
                iters += 1;
                let inst = *pattern.get(ind).unwrap();
                // dbg!(inst, cur_key, &map_map);
                let cur_val = maps.get(cur_key).unwrap();
                cur_key = cur_val.get(inst).unwrap();

                if cur_key.ends_with('Z') {
                    return iters;
                }
                ind = (ind + 1) % pattern.len(); // circular indexing
            }
        })
        .collect();

    Some(vec_lcm(ind_answers))
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
        // let result = part_two(&advent_of_code::template::read_file("examples", "08_2.txt"));
        assert_eq!(result, Some(6));
    }
}
