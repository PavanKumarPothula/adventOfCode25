#![allow(unused)]
advent_of_code::solution!(3);
fn best_face_value(number: Vec<u8>, face_value: u8) -> (usize, u8) {
    // Find the best face_value in the given range of numbers
    let (mut max_index, mut max_value) = (usize::MAX, u8::MIN);
    let right_limit = number.len() - face_value as usize;
    for (index, digit) in number[..right_limit].iter().enumerate() {
        if *digit > max_value {
            max_value = *digit;
            max_index = index;
        }
    }
    (max_index, max_value)
}

fn largest_joltage_overpowered(mut number: Vec<u8>, req_length: u8) -> u64 {
    // Recursively find best for the given req_length
    let mut output_vec: Vec<u8> = Vec::with_capacity(req_length as usize);
    let mut truncated_number = number.clone();
    let (mut out_index, mut out_best_value): (usize, u8);
    for i in (0..(req_length - 1)).rev() {
        (out_index, out_best_value) = best_face_value(truncated_number, i);
        truncated_number.drain(..out_index);
        output_vec.push(out_best_value);
    }
    0_u64
}

fn largest_joltage(number: Vec<u8>) -> u8 {
    // Go in reverse and find the following with the indices:
    //          - first_max_overall
    //          - second_max_overall
    //          - max_of_first_max_right
    // If the first max is the last digit, return (second_max_overall*10+ first max)
    // Else, return first_max*10 + max_of_first_max_right

    let mut iterator = number.iter().rev().cloned();
    let mut first_max_overall = iterator.next().unwrap_or_default();
    let (mut second_max_overall, mut max_of_first_max_right): (u8, u8) = (0, 0);
    let mut is_max_at_last_place = true;
    for x in iterator {
        if x >= first_max_overall {
            second_max_overall = first_max_overall;
            max_of_first_max_right = first_max_overall;
            first_max_overall = x;
            is_max_at_last_place = false;
        } else if x >= second_max_overall {
            second_max_overall = x;
        }
    }

    if is_max_at_last_place {
        (second_max_overall * 10) + first_max_overall
    } else {
        (first_max_overall * 10) + max_of_first_max_right
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_slices = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<u8>>());
    Some(input_slices.fold(0, |sum: u64, input_slice| {
        sum + largest_joltage(input_slice) as u64
    }))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_face_value() {
        assert_eq!(best_face_value(vec![1, 2, 3, 4, 5, 6], 2), (3, 4));
        assert_eq!(best_face_value(vec![1, 2, 3], 2), (0, 1));
    }

    #[test]
    fn test_largest_joltage() {
        assert_eq!(largest_joltage(vec![1, 2, 3, 4, 5, 4]), 54_u8);
        assert_eq!(largest_joltage(vec![1, 2, 3, 4, 5]), 45_u8);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
