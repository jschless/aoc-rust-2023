use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug, Clone)]
enum Instr {
    Accept, // leaf
    Reject, // leaf
    LessThan {
        var: usize,
        comp: i64,
        if_cond: Box<Instr>,
        else_cond: Box<Instr>,
    },
    GreaterThan {
        var: usize,
        comp: i64,
        if_cond: Box<Instr>,
        else_cond: Box<Instr>,
    },
}

impl Instr {
    fn from_vec(in_vec: &[&str], vec_map: HashMap<&str, Vec<&str>>) -> Self {
        let input = in_vec[0];
        if input.find(':').is_none() {
            match input {
                "A" => Instr::Accept,
                "R" => Instr::Reject,
                x => Instr::from_vec(&vec_map[x].clone(), vec_map),
            }
        } else {
            let (expr, if_cond) = input.split_once(':').unwrap();
            let var: usize = match expr.chars().next().unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!(),
            };
            let comp = expr[2..].parse::<i64>().unwrap();

            if input.find('>').is_some() {
                Instr::GreaterThan {
                    var,
                    comp,
                    if_cond: Box::new(Instr::from_vec(&[if_cond], vec_map.clone())),
                    else_cond: Box::new(Instr::from_vec(&in_vec[1..], vec_map.clone())),
                }
            } else {
                Instr::LessThan {
                    var,
                    comp,
                    if_cond: Box::new(Instr::from_vec(&[if_cond], vec_map.clone())),
                    else_cond: Box::new(Instr::from_vec(&in_vec[1..], vec_map.clone())),
                }
            }
        }
    }

    fn explore(&self, inp: &[i64]) -> bool {
        match self {
            Instr::Accept => true,
            Instr::Reject => false,
            Instr::GreaterThan {
                var,
                comp,
                if_cond,
                else_cond,
            } => {
                if inp[*var] > *comp {
                    if_cond.explore(inp)
                } else {
                    else_cond.explore(inp)
                }
            }
            Instr::LessThan {
                var,
                comp,
                if_cond,
                else_cond,
            } => {
                if inp[*var] < *comp {
                    if_cond.explore(inp)
                } else {
                    else_cond.explore(inp)
                }
            }
        }
    }

    fn n_combos(&self, ranges: &[(usize, usize)]) -> i64 {
        // maintain non-inclusive ranges for [x, m, a, s]
        // if reach an Accept, return number of combinations
        // if reach a reject, return 0

        match self {
            Instr::Accept => ranges.iter().map(|(l, u)| (u - l - 1) as i64).product(),
            Instr::Reject => 0,
            Instr::GreaterThan {
                var,
                comp,
                if_cond,
                else_cond,
            } => {
                let mut if_ranges = ranges.to_vec();
                let mut else_ranges = ranges.to_vec();
                if_ranges[*var] = (*comp as usize, ranges[*var].1);
                else_ranges[*var] = (ranges[*var].0, *comp as usize + 1);
                if_cond.n_combos(&if_ranges) + else_cond.n_combos(&else_ranges)
            }
            Instr::LessThan {
                var,
                comp,
                if_cond,
                else_cond,
            } => {
                let mut if_ranges = ranges.to_vec();
                let mut else_ranges = ranges.to_vec();
                else_ranges[*var] = (*comp as usize - 1, ranges[*var].1);
                if_ranges[*var] = (ranges[*var].0, *comp as usize);
                if_cond.n_combos(&if_ranges) + else_cond.n_combos(&else_ranges)
            }
        }
    }
}

fn parse_workflows(input: &str) -> Instr {
    let name_to_instr_list: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let begin_brack = line.find('{').unwrap();
            let name = &line[..begin_brack];
            let instr_list = &line[begin_brack + 1..line.len() - 1]
                .split(',')
                .collect::<Vec<&str>>();

            (name, instr_list.clone())
        })
        .collect();

    Instr::from_vec(&name_to_instr_list["in"], name_to_instr_list.clone())
}

pub fn part_one(input: &str) -> Option<i64> {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();
    let i = parse_workflows(workflows);

    let instructions: Vec<_> = inputs
        .lines()
        .map(|line| {
            let re =
                Regex::new(r".*x=(?P<xval>\d*),m=(?P<mval>\d*),a=(?P<aval>\d*),s=(?P<sval>\d*).*")
                    .unwrap();
            let cap = re.captures(line).unwrap();
            [
                cap["xval"].parse::<i64>().unwrap(),
                cap["mval"].parse::<i64>().unwrap(),
                cap["aval"].parse::<i64>().unwrap(),
                cap["sval"].parse::<i64>().unwrap(),
            ]
        })
        .collect();

    Some(
        instructions
            .iter()
            .cloned()
            .filter(|arr| i.explore(arr))
            .map(|arr| arr.iter().sum::<i64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let i = parse_workflows(workflows);
    let start_point = [(0, 4001), (0, 4001), (0, 4001), (0, 4001)];
    Some(i.n_combos(&start_point))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
