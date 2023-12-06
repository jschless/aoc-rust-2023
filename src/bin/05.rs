use std::str::FromStr;
advent_of_code::solution!(5);
#[allow(dead_code)]
#[derive(Debug, Clone)]

struct Mapping {
    dest_range_start: usize,
    source_range_start: usize,
    length: usize,
}

impl Mapping {
    fn map_seed(&self, seed: usize) -> usize {
        if seed < self.source_range_start || seed - self.source_range_start >= self.length {
            seed
        } else {
            self.dest_range_start + seed - self.source_range_start
        }
    }

    fn rev_map_seed(&self, seed: usize) -> usize {
        if seed < self.dest_range_start || seed - self.dest_range_start >= self.length {
            seed
        } else {
            self.source_range_start + seed - self.dest_range_start
        }
    }
}

impl FromStr for Mapping {
    type Err = &'static str; // Define the error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the input string and parse fields
        let parts: Vec<&str> = s.split_whitespace().collect();

        Ok(Mapping {
            dest_range_start: parts[0]
                .parse::<usize>()
                .expect("couldn't parse dest_start"),
            source_range_start: parts[1]
                .parse::<usize>()
                .expect("couldn't parse source_start"),
            length: parts[2].parse::<usize>().expect("couldn't parse range"),
        })
    }
}

fn remove_label(s: &str) -> &str {
    let col = s.find(':').unwrap();
    &s[col + 2..]
}

fn parse_input(input: &str) -> Option<(Vec<usize>, Vec<Vec<Mapping>>)> {
    if let Some((seeds, mappings)) = input.split_once("\n\n") {
        let new_seeds: Vec<usize> = remove_label(seeds)
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("expected usize str"))
            .collect();
        let new_mappings: Vec<Vec<Mapping>> = mappings
            .split("\n\n")
            .map(|mapping_group| {
                mapping_group
                    .lines()
                    .skip(1) // skip the label round
                    .map(|mapping| {
                        mapping
                            .parse::<Mapping>()
                            .expect("could not parse into mapping")
                    })
                    .collect()
            })
            .collect();
        Some((new_seeds, new_mappings))
    } else {
        None
    }
}

fn map_seed(seed: usize, mappings: &Vec<Mapping>) -> usize {
    for m in mappings {
        if m.map_seed(seed) != seed {
            return m.map_seed(seed);
        }
    }
    seed
}

fn rev_map_seed(seed: usize, mappings: &Vec<Mapping>) -> usize {
    for m in mappings {
        if m.rev_map_seed(seed) != seed {
            return m.rev_map_seed(seed);
        }
    }
    seed
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, mappings) = parse_input(input).unwrap();

    Some(
        seeds
            .iter()
            .map(|seed| {
                let mut ans: usize = *seed;
                for map in &mappings {
                    ans = map_seed(ans, map);
                }
                ans
            })
            .min()
            .unwrap() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // IDEA: Back propagate from each map... for each boundary condition :)

    let (seeds, mappings) = parse_input(input).unwrap();

    let mut rev_mappings: Vec<Vec<Mapping>> = mappings.clone();
    rev_mappings.reverse();

    let mut potential_start_seeds: Vec<usize> = Vec::new();

    for map_i in 0..rev_mappings.len() {
        let cur_map = &rev_mappings[map_i];
        // for all the source boundaries, let's reverse these to the end
        // dbg!(cur_map);
        let res: Vec<usize> = cur_map
            .iter()
            .map(|mapping| {
                let mut ans = mapping.source_range_start;

                // println!("Starting with SEED: {}", ans);
                for map in &rev_mappings[map_i + 1..] {
                    ans = rev_map_seed(ans, map);
                    // println!("Intermediate step: {}", ans);
                }
                ans
            })
            .collect();

        for i in res {
            potential_start_seeds.push(i);
        }
    }
    println!("Exploring {} possible seeds", potential_start_seeds.len());

    let mut new_seeds: Vec<usize> = Vec::new();
    for chunk in seeds.chunks(2) {
        match chunk {
            &[start, len] => {
                for i in &potential_start_seeds {
                    if i >= &start && i < &(start + len) {
                        new_seeds.push(*i);
                    }
                }
            }
            _ => {
                panic!()
            }
        }
    }

    Some(
        new_seeds
            .iter()
            .map(|seed| {
                let mut ans: usize = *seed;

                for map in &mappings {
                    ans = map_seed(ans, map);
                }
                ans
            })
            .min()
            .unwrap() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
