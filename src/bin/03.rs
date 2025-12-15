use std::u8;

advent_of_code::solution!(3);

fn largest_joltage(number: Vec<u8>) -> u8 {
    // Go in reverse and find the first max and max to right of first max, with their indices
    // If the first max is the last digit, return (second_max*10+ first max)
    // Else, return first_max*10 + second_max

    let mut iterator = number.iter().rev().cloned();
    let mut first_max = iterator.next().unwrap_or_default();
    let mut second_max: u8 = 0_u8;
    let mut is_max_at_last_place = true;
    for x in iterator {
        if x >= first_max {
            is_max_at_last_place = false;
            second_max = first_max;
            first_max = x;
        } else if x >= second_max {
            second_max = x;
        }
    }
    if is_max_at_last_place {
        (second_max * 10) + first_max
    } else {
        (first_max * 10) + second_max
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0_u64;
    for seq in input.split_ascii_whitespace() {
        let p = seq.chars().map(|x| x as u8);
        sum = sum + largest_joltage(p.collect()) as u64;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_joltage() {
        assert_eq!(largest_joltage(vec![1, 2, 3, 4, 5, 4]), 54_u8);
        assert_eq!(largest_joltage(vec![1, 2, 3, 4, 5]), 45_u8);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
