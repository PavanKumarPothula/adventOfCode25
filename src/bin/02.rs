use std::{cmp, u64};
advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    {
        input
            .split(',')
            .map(|pair| {
                let mut splits = pair.split('-');
                (
                    splits.next().unwrap().parse::<u64>().unwrap(),
                    splits.next().unwrap().parse::<u64>().unwrap(),
                )
            })
            .collect()
    }
}

pub fn split_into_valid_ranges(
    input_range: (u64, u64),
    two_repetations_only: bool,
) -> Option<Vec<(u64, u64)>> {
    //  Trims off of the impossible range of the given range.
    let (left, right) = (input_range.0, input_range.1);

    let (mut trimmed_left, trimmed_right): (u64, u64);
    let (left_digits, right_digits) = (left.to_string().len(), right.to_string().len());
    let mut out_vec = Vec::new();

    if two_repetations_only {
        // For exact two repeatations
        //      trimmed_left = left if digits(left) is even,
        //                   = 10^(log10(left)+1) else
        //
        //      trimmed_right= right if digits(right) is even,
        //                   = 10^(log10(right)-1)-1 else

        trimmed_left = if (left_digits % 2) == 0 {
            left
        } else {
            10_u64.pow((left_digits as u64).try_into().unwrap())
        };

        trimmed_right = if (right_digits % 2) == 0 {
            right
        } else {
            (10_u64.pow(((right_digits - 1) as u64).try_into().unwrap())) - 1
        };

        out_vec = vec![(trimmed_left, trimmed_right)];
    } else {
        // For usual repeatations
        //      trimmed_left = max(10,left)
        //      trimmed_right= None if right<=10, else right
        trimmed_left = cmp::max(10_u64, left);
        trimmed_right = right;
        let mut next_digit_max;
        loop {
            next_digit_max = 10_u64.pow(trimmed_left.to_string().len().try_into().unwrap()) - 1;
            if next_digit_max >= trimmed_right {
                out_vec.append(&mut vec![(trimmed_left, trimmed_right)]);
                break;
            } else {
                out_vec.append(&mut vec![(trimmed_left, next_digit_max)]);
                trimmed_left = next_digit_max + 1;
            }
        }
    }

    return if (trimmed_left > trimmed_right) || trimmed_right <= 10 {
        None
    } else {
        Some(out_vec)
    };
}
pub fn possible_rep_number_sum(input_range: (u64, u64)) -> Option<Vec<u64>> {
    // This should call get_sum_of_invalid_ids_for_range for each possibility
    None
}

pub fn get_sum_of_invalid_ids_for_range(input_range: (u64, u64)) -> u64 {
    // For range [AB,CD],
    //      AB <= ((10^i)+1)x <= CD
    //      where the i = (number of digits/2)
    //
    //  x E [n,m], sum of all such numbers is:
    //      ((10^i)+1)*((m^2 + m)/2 - (n^2 + n)/2)
    //          where n =   A if A==B OR max(A,B)=A, else A+1
    //            and m = C-1 if max(C,D)=C, else C
    let (left, right) = (input_range.0, input_range.1);
    let i = ((left.to_string().len()) / 2) as u64;
    let (a, b) = (
        left / (10_u64.pow(i.try_into().unwrap())),
        left % (10_u64.pow(i.try_into().unwrap())),
    );
    let (c, d) = (
        right / (10_u64.pow(i.try_into().unwrap())),
        right % (10_u64.pow(i.try_into().unwrap())),
    );
    let (n, m);

    if a >= b {
        n = a;
    } else {
        n = a + 1;
    }
    if c > d {
        m = c - 1;
    } else {
        m = c;
    }
    if m < n {
        return 0;
    } else {
        return ((10_u64.pow(i.try_into().unwrap())) + 1) * (((m + n) * (m - n + 1)) / 2);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grand_total: u64 = 0;
    for _range in parse_input(input) {
        let valid_ones = split_into_valid_ranges(_range, true);
        if valid_ones.is_none() {
            continue;
        } else {
            for _valid_range in valid_ones.unwrap() {
                grand_total = grand_total + get_sum_of_invalid_ids_for_range(_valid_range);
            }
        }
    }
    return Some(grand_total);
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_input("1-2,34-56,789-890"),
            vec![(1, 2), (34, 56), (789, 890)]
        );
    }

    #[test]
    fn test_splitter_for_part_one() {
        assert_eq!(
            split_into_valid_ranges((10, 99), true),
            Some(vec![(10, 99)])
        );
        assert_eq!(split_into_valid_ranges((110, 990), true), None);
        assert_eq!(
            split_into_valid_ranges((9123, 12332), true),
            Some(vec![(9123, 9999)])
        );
        assert_eq!(split_into_valid_ranges((1, 8), true), None);
    }

    #[test]
    fn test_splitter_for_part_two() {
        assert_eq!(
            split_into_valid_ranges((1, 98), false),
            Some(vec![(10, 98)])
        );
        assert_eq!(
            split_into_valid_ranges((110, 990), false),
            Some(vec![(110, 990)])
        );
        assert_eq!(
            split_into_valid_ranges((9123, 12332), false),
            Some(vec![(9123, 9999), (10000, 12332)])
        );

        assert_eq!(
            split_into_valid_ranges((99, 999), false),
            Some(vec![(99, 99), (100, 999)])
        );
        assert_eq!(split_into_valid_ranges((1, 8), false), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
