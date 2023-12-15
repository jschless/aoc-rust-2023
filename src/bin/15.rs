advent_of_code::solution!(15);

fn hash(ascii_vec: &[u8]) -> u32 {
    // Determined the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    let mut acc: u32 = 0;
    for ascii_val in ascii_vec {
        acc += *ascii_val as u32;
        acc *= 17;
        acc %= 256;
    }
    acc
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split(',')
            .map(|tok| tok.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .map(|v| hash(&v))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<(&str, u8)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for tok in input.trim().split(',') {
        let mut is_equals = false;
        let label = if let Some(x) = tok.find('=') {
            is_equals = true;
            &tok[..x]
        } else {
            &tok[..tok.len() - 1]
        };

        let box_num = hash(&label.chars().map(|c| c as u8).collect::<Vec<u8>>());
        let b = boxes.get_mut(box_num as usize).unwrap();

        let mut location = 1919;
        for (i, (lab, _)) in b.iter().enumerate() {
            if lab == &label {
                location = i;
                break;
            }
        }

        if is_equals {
            let focal_len: u8 = tok[tok.find('=').unwrap() + 1..].parse::<u8>().unwrap();
            if location != 1919 {
                let _ = std::mem::replace(&mut b[location], (label, focal_len));
            } else {
                b.push((label, focal_len));
            }
        } else if location != 1919 {
            b.remove(location);
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_i, lens_vec)| {
                lens_vec
                    .iter()
                    .enumerate()
                    .map(|(slot_i, (_, focal_len))| {
                        (1 + box_i) * (1 + slot_i) * *focal_len as usize
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("HASH");
        assert_eq!(result, Some(52));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
