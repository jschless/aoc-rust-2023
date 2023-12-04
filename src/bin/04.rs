use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn semi_pow(i: u32) -> u32 {
    if i == 0 {
        0
    } else {
        2_u32.pow(i - 1)
    }
}

fn get_intersections(input: &str) -> Vec<usize> {
    // returns size of intersection of each card's winning numbers and numbers you have
    input
        .lines()
        .map(|line| {
            let col = line.find(':').unwrap();
            if let Some((winning, have)) = &line[col + 2..].split_once('|') {
                let win_set: HashSet<u32> = winning
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
                let have_set: HashSet<u32> = have
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
                win_set.intersection(&have_set).count()
            } else {
                0
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        get_intersections(input)
            .iter()
            .map(|size| semi_pow(*size as u32))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<usize> = get_intersections(input);

    let mut counts_map: HashMap<usize, usize> = HashMap::new();

    for (i, card) in cards.iter().enumerate() {
        let card_num = i + 1;
        let cur_count = *counts_map.entry(card_num).or_insert(1);
        for j in card_num + 1..=card_num + card {
            let new_count = counts_map.entry(j).or_insert(1);
            *new_count += cur_count;
        }
    }

    Some(counts_map.values().sum::<usize>() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
