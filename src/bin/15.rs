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
            .replace('\n', "")
            .split(',')
            .map(|tok| tok.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .map(|v| hash(&v))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&"HASH");
        assert_eq!(result, Some(52));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
