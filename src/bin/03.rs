advent_of_code::solution!(3);

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
