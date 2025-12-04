use ilog::IntLog;
advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    return input
        .split(',')
        .map(|pair| pair.split('-').map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();
}

pub fn split_into_valid_ranges(input_range: Vec<u64>) -> Option<Vec<u64>> {
    //  Trims off of the impossible range of the given range.
    //
    //      trimmed_left = left if digits(left) is even,
    //                   = 10^(log10(left)+1) else
    //
    //      trimmed_right= right if digits(right) is even,
    //                   = 10^(log10(right)-1)-1 else

    let (left, right) = (input_range[0], input_range[1]);

    let (trimmed_left, trimmed_right): (u64, u64);
    let (left_digits, right_digits) = (left.log10() + 1, right.log10() + 1);

    if (left_digits % 2) == 0 {
        trimmed_left = left;
    } else {
        trimmed_left = 10_u64.pow((left_digits as u64).try_into().unwrap());
    }

    if (right_digits % 2) == 0 {
        trimmed_right = right;
    } else {
        trimmed_right = (10_u64.pow(((right_digits - 1) as u64).try_into().unwrap())) - 1;
    }

    if trimmed_left > trimmed_right {
        return None;
    } else {
        return Some(vec![trimmed_left, trimmed_right]);
    }
}

pub fn get_sum_of_invalid_ids_for_range(input_range: Option<Vec<u64>>) -> u64 {
    // For range [AB,CD],
    //      AB <= ((10^i)+1)x <= CD
    //      where the i = (number of digits/2)
    //
    //  x E [n,m], sum of all such numbers is:
    //      ((10^i)+1)*((m^2 + m)/2 - (n^2 + n)/2)
    //          where n =   A if A==B OR max(A,B)=A, else A+1
    //            and m = C-1 if max(C,D)=C, else C
    let _input;
    if input_range == None {
        return 0;
    } else {
        _input = input_range.unwrap();
    }
    let (left, right) = (_input[0], _input[1]);
    let i = ((left.log10() + 1) / 2) as u64;
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
        let valid_ones = split_into_valid_ranges(_range);
        grand_total = grand_total + get_sum_of_invalid_ids_for_range(valid_ones);
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_splitter() {
        assert_eq!(split_into_valid_ranges(vec!(10, 99)), Some(vec!(10, 99)));
        assert_eq!(split_into_valid_ranges(vec!(110, 990)), None);
        assert_eq!(
            split_into_valid_ranges(vec!(9123, 12332)),
            Some(vec!(9123, 9999))
        );
        assert_eq!(split_into_valid_ranges(vec!(1, 8)), None);
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_input("1-2,34-56,789-890"),
            vec![vec![1, 2], vec![34, 56], vec![789, 890]]
        );
    }
}
