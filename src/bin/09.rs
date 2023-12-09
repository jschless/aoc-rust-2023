advent_of_code::solution!(9);

fn predict(mut vec: Vec<i32>, next: bool) -> i32 {
    // if next is true, predict next, else predict previous
    let mut elems: Vec<i32> = Vec::new();
    while vec.iter().any(|x| *x != 0) {
        elems.push(if next {
            *vec.last().unwrap()
        } else {
            *vec.first().unwrap()
        });
        vec = vec.windows(2).map(|arr| arr[1] - arr[0]).collect();
    }

    elems
        .iter()
        .rev()
        .cloned()
        .reduce(|acc, e| if next { acc + e } else { e - acc })
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        parse_input(input)
            .iter()
            .cloned()
            .map(|v| predict(v, true))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        parse_input(input)
            .iter()
            .cloned()
            .map(|v| predict(v, false))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
