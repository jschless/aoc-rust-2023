use regex::Regex;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
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
            // .take(1)
            .map(|(in_vec, pattern)| gen_matches(in_vec, pattern))
            .sum(),
    )
}

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
    // TODO figure out if it is possible given number of wildcards left
    let ans = get_counts(v);

    let mut last_neq = false; // whether we've already encountered a neq
    let mut ret = true;
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

    if pat.len() < ans.len() {
        return false;
    } else if pat.len() > ans.len() {
        diff += pat[ans.len()..].iter().sum::<usize>();

        if diff > n_chars {
            // dbg!(&v, &pat, diff, n_chars);
            return false;
        }
    }
    // dbg!(&v, &ans, ret, &pat);
    ret
}

fn gen_matches(v: Vec<char>, pattern: &Vec<usize>) -> u32 {
    let mut to_explore: Vec<Vec<char>> = Vec::new();
    let mut options: Vec<Vec<char>> = Vec::new();

    to_explore.push(Vec::new());
    let mut total_explored = 0;
    let N_CHARS = v.len();
    for c in v {
        while let Some(mut builder) = to_explore.pop() {
            if check(&builder, pattern, N_CHARS) {
                total_explored += 1;
                match c {
                    '?' => {
                        let mut new_builder = builder.clone();
                        new_builder.push('p');
                        options.push(new_builder);
                        let mut new_builder = builder.clone();
                        new_builder.push('#');
                        total_explored += 1;
                        options.push(new_builder);
                    }
                    '.' => {
                        builder.push('p');
                        options.push(builder);
                    }
                    c => {
                        builder.push(c);
                        options.push(builder);
                    }
                }
            }
        }
        to_explore = options.clone();
        options = Vec::new();
    }

    let matches: Vec<Vec<char>> = to_explore
        .iter()
        .filter(|v| check_final(v, pattern))
        .cloned()
        .collect();
    // dbg!(matches.len(), pattern);
    // if matches.len() == 17 {
    //     dbg!(matches, pattern);
    // }

    // start point: total_explored = 452 for test, 915016 for run
    // adding early termination based  on remaining length: total_explored = 440 for test, 416134 for run

    // total_explored
    matches.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
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
}
