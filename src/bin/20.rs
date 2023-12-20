use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Module {
    name: String,
    next: Vec<String>,
    mod_type: ModuleType,
    high: bool,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop { on: bool },
    Conjunction { last: HashMap<String, bool> },
    Broadcast,
}

impl Module {
    fn from_str(input: &str) -> Self {
        let (mod_type, next) = input.split_once(" -> ").unwrap();
        let next = next
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if mod_type == "broadcaster" {
            Module {
                name: mod_type.to_string(),
                next,
                high: false,
                mod_type: ModuleType::Broadcast,
            }
        } else if mod_type.find('%').is_some() {
            Module {
                name: mod_type[1..].to_string(),
                next,
                high: false,
                mod_type: ModuleType::FlipFlop { on: false },
            }
        } else {
            Module {
                name: mod_type[1..].to_string(),
                next,
                high: false,
                mod_type: ModuleType::Conjunction {
                    last: HashMap::new(),
                },
            }
        }
    }

    fn pulse(&mut self, high: bool) -> Option<bool> {
        match self.mod_type.clone() {
            ModuleType::Broadcast => Some(high),
            ModuleType::FlipFlop { on } => {
                if !high {
                    self.mod_type = ModuleType::FlipFlop { on: !on };
                    Some(!on)
                } else {
                    None
                }
            }
            ModuleType::Conjunction { last } => Some(!last.iter().all(|(_k, v)| *v)),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mods: Vec<Module> = input.lines().map(Module::from_str).collect();
    let mut name_to_prev: HashMap<String, Vec<String>> = HashMap::new();
    name_to_prev.insert("output".to_string(), Vec::new());
    let name_to_mod: HashMap<String, Module> = mods
        .iter()
        .cloned()
        .map(|m| {
            name_to_prev.insert(m.name.clone(), Vec::new());
            (m.name.clone(), m.clone())
        })
        .collect();

    // initialize all last maps
    for prev in &mods {
        for n in &prev.next {
            if name_to_prev.contains_key(n) {
                name_to_prev.get_mut(n).unwrap().push(prev.name.clone());
            } else {
                name_to_prev.insert(n.to_string(), vec![prev.name.clone()]);
            }
        }
    }

    for m in mods.iter_mut() {
        if let ModuleType::Conjunction { ref mut last } = m.mod_type {
            *last = name_to_prev
                .get(&m.name)
                .unwrap()
                .iter()
                .map(|s| (s.clone(), false))
                .collect::<HashMap<String, bool>>();
        }
    }

    let mut name_to_mod: HashMap<String, Module> = mods
        .iter()
        .cloned()
        .map(|m| (m.name.clone(), m.clone()))
        .collect();
    let mut count_low = 0;
    let mut count_high = 0;

    const N_PUSHES: usize = 1000;

    for _ in 0..N_PUSHES {
        let mut to_pulse: VecDeque<(Module, bool, String)> = VecDeque::new();
        to_pulse.push_back((
            name_to_mod.get("broadcaster").unwrap().clone(),
            false,
            "button".to_string(),
        ));
        count_low += 1;
        while let Some((mut m, pulse, prev)) = to_pulse.pop_front() {
            let saved_name = m.name.clone();
            if let ModuleType::Conjunction { ref mut last } = m.mod_type {
                last.insert(prev.to_string(), pulse);
            }

            // dbg!(&m, pulse, &prev);
            // 1. send a pulse to module
            if let Some(next_pulse) = m.pulse(pulse) {
                // 2. send a pulse to neighbors
                // dbg!("propogating", next_pulse);
                name_to_mod.insert(m.name.clone(), m.clone());
                for v in &m.next {
                    if next_pulse {
                        // println!("high pulse to {}", { v });
                        count_high += 1;
                    } else {
                        // println!("low pulse to {}", { v });
                        count_low += 1;
                    }

                    if name_to_mod.contains_key(v) {
                        to_pulse.push_back((
                            name_to_mod.get(v).unwrap().clone(),
                            next_pulse,
                            saved_name.clone(),
                        ));
                    }
                }
            }
        }
    }
    dbg!(count_low, count_high);

    Some(count_low * count_high)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let result = part_one(&fs::read_to_string("./data/examples/20_2.txt").unwrap());
        assert_eq!(result, Some(32000000));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
