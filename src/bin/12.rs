use std::collections::HashMap;
advent_of_code::solution!(12);

fn get_counts(v: &Vec<char>) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();
    let mut count = 0;
    for &c in v {
        if c == '#' {
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

fn check_final(v: &Vec<char>, pat: &Vec<usize>) -> bool {
    let ans = get_counts(v);
    if ans.len() != pat.len() {
        return false;
    }
    ans.iter().zip(pat.iter()).all(|(a, b)| a == b)
}

fn check(v: &Vec<char>, pat: &Vec<usize>, n_chars: usize) -> bool {
    let ans = get_counts(v);

    let mut last_neq = false; // whether we've already encountered a neq
    let mut diff = 0;
    for (ans, pat) in ans.iter().zip(pat.iter()) {
        if last_neq || ans > pat {
            // an earlier thing didn't match
            return false;
        } else if ans < pat {
            last_neq = true;
            diff = pat - ans;
        }
    }
    if last_neq && v.last().unwrap() == &'.' {
        return false;
    }

    if pat.len() > ans.len() {
        diff += pat[ans.len()..].iter().sum::<usize>(); // add up the remaining #s
        diff += pat.len() - ans.len(); // add up the minimum 1 periods to remove
        diff -= 1; //don't need a period at the end
    }

    !(pat.len() < ans.len() || (pat.len() > ans.len() && diff > n_chars))
}

fn remainder(v: &Vec<char>, pat: &[usize]) -> Vec<usize> {
    // given a matching vector, returns how much of the pattern is left
    // the memo could look like this
    // last char, remaining vec, remaining pattern (would include)
    let ans = get_counts(v);
    pat.iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| x - ans.get(i).unwrap_or(&0))
        .filter(|x| x != &0)
        .collect()
}

fn is_partial(v: &Vec<char>, pat: &[usize]) -> bool {
    // returns true if the vector is currently matching a pattern
    let ans = get_counts(v);
    pat.iter().zip(ans.iter()).any(|(a, b)| b < a)
}

fn get_next(
    remaining: &[char],
    builder: Vec<char>,
    pattern: &Vec<usize>,
    memo: &mut HashMap<(bool, bool, Vec<char>, Vec<usize>), u64>,
) -> u64 {
    if !check(&builder, pattern, remaining.len()) {
        return 0;
    }

    let last_hash = builder.last().cloned().unwrap_or('.') == '#';
    let remaining_pattern = remainder(&builder, pattern); // pattern matches
    let ongoing_match = is_partial(&builder, pattern);
    let key = (
        last_hash,
        ongoing_match,
        remaining.to_vec(),
        remaining_pattern.to_vec(),
    );
    if memo.contains_key(&key) {
        *memo.get(&key).unwrap()
    } else {
        let cap = match remaining {
            [c, rest @ ..] => match *c {
                '?' => {
                    let mut temp_vec = builder.clone();
                    temp_vec.push('#');
                    let mut temp_vec_2 = builder.clone();
                    temp_vec_2.push('.');
                    get_next(rest, temp_vec, pattern, memo)
                        + get_next(rest, temp_vec_2, pattern, memo)
                }
                c => {
                    let mut temp_vec = builder.clone();
                    temp_vec.push(c);
                    get_next(rest, temp_vec, pattern, memo)
                }
            },
            [] => {
                if check_final(&builder, pattern) {
                    1
                } else {
                    0
                }
            }
        };

        memo.insert(key, cap);
        cap
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let patterns: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .unwrap()
                .1
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut memo: HashMap<(bool, bool, Vec<char>, Vec<usize>), u64> = HashMap::new();
    Some(
        input
            .lines()
            .map(|line| {
                line.split_once(' ')
                    .unwrap()
                    .0
                    .chars()
                    .collect::<Vec<char>>()
            })
            .zip(patterns.iter())
            .map(|(in_vec, pattern)| get_next(&in_vec, Vec::new(), pattern, &mut memo))
            .sum::<u64>(),
    )
}

fn dup_chars(in_vec: &[char]) -> Vec<char> {
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

pub fn part_two(input: &str) -> Option<u64> {
    let patterns: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let temp_vec: Vec<usize> = line
                .split_once(' ')
                .unwrap()
                .1
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut new_vec = Vec::new();
            for _ in 0..5 {
                for i in &temp_vec {
                    new_vec.push(*i);
                }
            }
            new_vec
        })
        .collect::<Vec<Vec<usize>>>();
    let mut memo: HashMap<(bool, bool, Vec<char>, Vec<usize>), u64> = HashMap::new();
    Some(
        input
            .lines()
            .map(|line| {
                let single_vec = line
                    .split_once(' ')
                    .unwrap()
                    .0
                    .chars()
                    .collect::<Vec<char>>();
                dup_chars(&single_vec)
            })
            .zip(patterns.iter())
            .map(|(in_vec, pattern)| get_next(&in_vec, Vec::new(), pattern, &mut memo))
            .sum(),
    )
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
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_checker() {
        let result = check(
            &vec!['.', '#', '#', '#', '.', '.', '.', '.', '#', '.'],
            &vec![3, 2, 1],
            2,
        );
        assert_eq!(result, false);

        let result = check(
            &vec![
                '#', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#',
            ],
            &vec![1, 3, 3, 2],
            2,
        );
        assert_eq!(result, false);
    }
}
