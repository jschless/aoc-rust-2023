use petgraph::algo::{astar, kosaraju_scc};
use petgraph::graph::{NodeIndex, UnGraph};
use rand::prelude::*;

use std::collections::HashMap;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iids: HashMap<String, u32> = HashMap::new();
    let mut n = 0;
    let mut g = UnGraph::<_, ()>::new_undirected();
    let comps: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            if !iids.contains_key(left) {
                iids.insert(left.to_string(), n);
                g.add_node(left);
                n += 1;
            }
            let mut temp: Vec<_> = Vec::new();
            for s in right.trim().split(' ') {
                if !iids.contains_key(&s.to_string()) {
                    iids.insert(s.to_string(), n);
                    g.add_node(s);
                    n += 1;
                }
                temp.push((iids[left], iids[s]));
            }
            temp
        })
        .collect();

    g.extend_with_edges(comps);

    let mut rng = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    const N_RAND: usize = 300;
    let mut node_count: HashMap<NodeIndex, usize> = HashMap::new();

    for (i, j) in (0..N_RAND)
        .map(|_| rng.gen_range(0..n))
        .zip((0..N_RAND).map(|_| rng2.gen_range(0..n)))
    {
        let s = NodeIndex::new(i as usize);
        let e = NodeIndex::new(j as usize);
        if let Some((_, path)) = astar(&g, s, |f| f == e, |_| 1, |_| 0) {
            for node_ind in path {
                node_count
                    .entry(node_ind)
                    .and_modify(|x| *x += 1)
                    .or_insert(0);
            }
        }
    }

    let mut pairs: Vec<_> = node_count.iter().collect();
    pairs.sort_by(|&(_, a), &(_, b)| b.cmp(a));

    for i in 0..6 {
        for j in i..6 {
            if let Some(edge) = g.find_edge(*pairs[i].0, *pairs[j].0) {
                // println!(
                //     "Removing edge from {} to {} ",
                //     g.node_weight(*pairs[i].0).unwrap(),
                //     g.node_weight(*pairs[j].0).unwrap()
                // );
                g.remove_edge(edge);
            }
        }
    }
    Some(kosaraju_scc(&g).iter().map(|v| v.len() as u32).product())
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
