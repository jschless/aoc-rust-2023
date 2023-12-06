use std::str::FromStr;
advent_of_code::solution!(5);
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]

struct MapEntry {
    dest_range_start: usize,
    source_range_start: usize,
    length: usize,
}

impl MapEntry {
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

impl FromStr for MapEntry {
    type Err = &'static str; // Define the error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        Ok(MapEntry {
            dest_range_start: parts[0].parse::<usize>().unwrap(),
            source_range_start: parts[1].parse::<usize>().unwrap(),
            length: parts[2].parse::<usize>().unwrap(),
        })
    }
}

impl FromStr for Mapping {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mapping(
            s.lines()
                .skip(1) // skip the label
                .map(|mapping| mapping.parse::<MapEntry>().unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Mapping(Vec<MapEntry>);

impl Mapping {
    fn map_seed(&self, seed: usize) -> usize {
        for m in &self.0 {
            if m.map_seed(seed) != seed {
                return m.map_seed(seed);
            }
        }
        seed
    }

    fn rev_map_seed(&self, seed: usize) -> usize {
        for m in &self.0 {
            if m.rev_map_seed(seed) != seed {
                return m.rev_map_seed(seed);
            }
        }
        seed
    }
}

fn parse_input(input: &str) -> Option<(Vec<usize>, Vec<Mapping>)> {
    if let Some((seeds, mappings)) = input.split_once("\n\n") {
        let new_seeds: Vec<usize> = seeds[7..] // start where the numbers are
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("expected usize str"))
            .collect();
        let new_mappings: Vec<Mapping> = mappings
            .split("\n\n")
            .map(|mapping| mapping.parse().unwrap())
            .collect();
        Some((new_seeds, new_mappings))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, mappings) = parse_input(input).unwrap();

    Some(
        seeds
            .iter()
            .map(|seed| mappings.iter().fold(*seed, |acc, map| map.map_seed(acc)))
            .min()
            .unwrap() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // IDEA: Back propagate from each map... for each boundary condition :)

    let (seeds, mappings) = parse_input(input).unwrap();

    let rev_mappings: Vec<Mapping> = mappings.iter().cloned().rev().collect();

    let mut potential_start_seeds: Vec<usize> = Vec::new();

    for map_i in 0..rev_mappings.len() {
        rev_mappings[map_i]
            .0
            .iter()
            .map(|mapping| {
                rev_mappings[map_i + 1..]
                    .iter()
                    .fold(mapping.source_range_start, |acc, map| map.rev_map_seed(acc))
            })
            .map(|x| potential_start_seeds.push(x))
            .min(); // force iter to eval
    }

    // only test the seeds that are in valid ranges
    let new_seeds: Vec<usize> = seeds
        .chunks(2)
        .flat_map(|chunk| match chunk {
            &[start, len] => potential_start_seeds.iter().filter_map(move |i| {
                if *i >= start && *i < start + len {
                    Some(*i)
                } else {
                    None
                }
            }),
            _ => panic!(),
        })
        .collect();

    println!(
        "Explored {} of {} possible seeds",
        new_seeds.len(),
        potential_start_seeds.len()
    );
    Some(
        new_seeds
            .iter()
            .map(|seed| mappings.iter().fold(*seed, |acc, map| map.map_seed(acc)))
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
