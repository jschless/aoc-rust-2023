use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Record {
    builder: Vec<char>,
    pattern: Vec<usize>,
    start_str: Vec<char>,
}

impl Record {
    fn countify(&self) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();
        let mut count = 0;
        for c in &self.builder {
            if c == &'#' {
                count += 1;
            } else if count > 0 {
                ans.push(count);
                count = 0;
            }
        }
        if count > 0 {
            ans.push(count);
        }

        ans
    }

    fn remainder(&self) -> Vec<usize> {
        // given a matching vector, returns how much of the pattern is left
        let ans = self.countify();
        self.pattern
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| x - ans.get(i).unwrap_or(&0))
            .filter(|x| x != &0)
            .collect()
    }

    fn remaining_str(&self) -> &[char] {
        &self.start_str[self.builder.len()..]
    }

    fn check(&self) -> bool {
        let counts = self.countify();
        let n_chars = self.start_str.len() - self.builder.len();
        if n_chars == 0 {
            // it's the right length, check everything
            if counts.len() != self.pattern.len() {
                return false;
            }
            return counts.iter().zip(self.pattern.iter()).all(|(a, b)| a == b);
        }

        let mut last_neq = false; // whether we've already encountered a neq
        let mut diff = 0;
        for (count, pat) in counts.iter().zip(self.pattern.iter()) {
            if last_neq || count > pat {
                // an earlier thing didn't match, so this next thing can't
                return false;
            } else if count < pat {
                last_neq = true;
                diff = pat - count;
            }
        }
        if last_neq && self.builder.last().unwrap() == &'.' {
            //
            return false;
        }

        if self.pattern.len() > counts.len() {
            diff += self.pattern[counts.len()..].iter().sum::<usize>(); // add up the remaining #s
            diff += self.pattern.len() - counts.len(); // add up the minimum 1 periods to remove
            diff -= 1; //don't need a period at the end
        }

        !(self.pattern.len() < counts.len()
            || (self.pattern.len() > counts.len() && diff > n_chars))
    }

    fn is_partial(&self) -> bool {
        // returns true if the vector is currently matching a pattern
        self.pattern
            .iter()
            .zip(self.countify().iter())
            .any(|(a, b)| b < a)
    }

    fn _to_key(&self) -> (bool, bool, &[char], Vec<usize>) {
        (
            (self.builder.last().cloned().unwrap_or('.') == '#'),
            self.is_partial(),
            self.remaining_str(),
            self.remainder(),
        )
    }
}

impl Hash for Record {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.builder.last().cloned().unwrap_or('.') == '#').hash(state);
        self.is_partial().hash(state);
        self.remainder().hash(state);
        self.remaining_str().hash(state);
    }
}

fn recurse(record: Record, memo: &mut HashMap<Record, u64>) -> u64 {
    if !record.check() {
        //current pattern isn't possible
        return 0;
    }

    match memo.get(&record) {
        Some(val) => *val,
        None => {
            let cap = match record.remaining_str() {
                [c, _rest @ ..] => match *c {
                    '?' => {
                        let mut next_record = record.clone();
                        next_record.builder.push('#');
                        let mut next_record_2 = record.clone();
                        next_record_2.builder.push('.');
                        recurse(next_record, memo) + recurse(next_record_2, memo)
                    }
                    c => {
                        let mut next_record = record.clone();
                        next_record.builder.push(c);
                        recurse(next_record, memo)
                    }
                },
                [] => {
                    if record.check() {
                        1
                    } else {
                        0
                    }
                }
            };

            memo.insert(record, cap);
            cap
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let records: Vec<Record> = input
        .lines()
        .map(|line| {
            let (start_str, pattern) = line.split_once(' ').unwrap();
            Record {
                start_str: start_str.chars().collect(),
                builder: Vec::new(),
                pattern: pattern
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect();

    let mut memo: HashMap<Record, u64> = HashMap::new();

    Some(
        records
            .iter()
            .map(|r| recurse(r.clone(), &mut memo))
            .sum::<u64>(),
    )
    // dbg!(memo);
    // Some(0)
}

fn _dup_chars(in_vec: &[char]) -> Vec<char> {
    let mut new_vec: Vec<char> = Vec::new();
    for _ in 0..5 {
        for &x in in_vec {
            new_vec.push(x);
        }
        new_vec.push('?');
    }
    new_vec.pop();
    new_vec
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    // #[test]
    // fn test_checker() {
    //     let result = check(
    //         &vec!['.', '#', '#', '#', '.', '.', '.', '.', '#', '.'],
    //         &vec![3, 2, 1],
    //         2,
    //     );
    //     assert_eq!(result, false);

    //     let result = check(
    //         &vec![
    //             '#', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#',
    //         ],
    //         &vec![1, 3, 3, 2],
    //         2,
    //     );
    //     assert_eq!(result, false);
    // }
}
