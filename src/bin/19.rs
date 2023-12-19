use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug)]
enum Instr {
    Accept,
    Reject,
    Goto {
        dest: String,
    },
    LessThan {
        var: char,
        comp: u32,
        else_cond: Box<Instr>,
    },
    GreaterThan {
        var: char,
        comp: u32,
        else_cond: Box<Instr>,
    },
}

impl Instr {
    fn from_str(input: &str) -> Self {
        if input.find(':').is_none() {
            match input {
                "A" => Instr::Accept,
                "R" => Instr::Reject,
                x => Instr::Goto {
                    dest: x.to_string(),
                },
            }
        } else {
            let (expr, else_cond) = input.split_once(':').unwrap();
            let var: char = expr.chars().next().unwrap();
            let comp = expr[2..].parse::<u32>().unwrap();

            if input.find('>').is_some() {
                Instr::GreaterThan {
                    var,
                    comp,
                    else_cond: Box::new(Instr::from_str(else_cond)),
                }
            } else {
                Instr::LessThan {
                    var,
                    comp,
                    else_cond: Box::new(Instr::from_str(else_cond)),
                }
            }
        }
    }
}

fn parse_workflows(input: &str) -> (Vec<Vec<Instr>>, HashMap<&str, usize>) {
    let mut name_to_workflow: HashMap<&str, usize> = HashMap::new();
    let v: Vec<Vec<Instr>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let begin_brack = line.find('{').unwrap();
            let name = &line[..begin_brack];
            name_to_workflow.insert(name, i);
            let instr_list = &line[begin_brack + 1..line.len() - 1];
            instr_list
                .split(',')
                .map(Instr::from_str)
                .collect::<Vec<Instr>>()
        })
        .collect();
    (v, name_to_workflow)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();
    let (v, name_to_workflow) = parse_workflows(workflows);
    let instructions: Vec<_> = inputs
        .lines()
        .map(|line| {
            let re =
                Regex::new(r".*x=(?P<xval>\d*),m=(?P<mval>\d*),a=(?P<aval>\d*),s=(?P<sval>\d*).*")
                    .unwrap();
            let cap = re.captures(line).unwrap();
            (
                cap["xval"].parse::<u32>().unwrap(),
                cap["mval"].parse::<u32>().unwrap(),
                cap["aval"].parse::<u32>().unwrap(),
                cap["sval"].parse::<u32>().unwrap(),
            )
        })
        .collect();

    Some(
        instructions
            .iter()
            .cloned()
            .filter(|tup| explore(*tup, &v, &name_to_workflow))
            .map(|(x, m, a, s)| x + m + a + s)
            .sum(),
    )
}

fn explore(
    (x, m, a, s): (u32, u32, u32, u32),
    v: &[Vec<Instr>],
    ma: &HashMap<&str, usize>,
) -> bool {
    let mut vec = &v[ma["in"]];
    let mut i = 0;
    let mut next_instr = &vec[i];
    loop {
        match next_instr {
            Instr::Accept => {
                return true;
            }
            Instr::Reject => {
                return false;
            }
            Instr::Goto { dest } => {
                let s = &dest as &str;
                vec = &v[ma[s]];
                i = 0;
                next_instr = &vec[i];
            }
            Instr::GreaterThan {
                var,
                comp,
                else_cond,
            } => {
                let val = match var {
                    'x' => x,
                    'm' => m,
                    'a' => a,
                    's' => s,
                    _ => panic!("found {}", var),
                };
                if val > *comp {
                    next_instr = else_cond;
                } else {
                    i += 1;
                    next_instr = &vec[i];
                }
            }
            Instr::LessThan {
                var,
                comp,
                else_cond,
            } => {
                let val = match var {
                    'x' => x,
                    'm' => m,
                    'a' => a,
                    's' => s,
                    _ => panic!("found {}", var),
                };
                if val < *comp {
                    next_instr = else_cond;
                } else {
                    i += 1;
                    next_instr = &vec[i];
                }
            }
        }
    }
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
