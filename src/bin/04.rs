advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| if char == '@' { 1_u8 } else { 0_u8 })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut output = 0_u64;
    matrix.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, digit)| {
            if *digit == 1
                && (matrix.get(row_index - 1)
                .and_then(|row|row.get(column_index)).unwrap_or(&0_u8)
                    + matrix.get(row_index + 1)
                .and_then(|row|row.get(column_index) ).unwrap_or(&0_u8)
                    + matrix.get(row_index)
                .and_then(|row|row.get(column_index - 1) ).unwrap_or(&0_u8)
                    + matrix.get(row_index)
                .and_then(|row|row.get(column_index + 1) ).unwrap_or(&0_u8)
                )<4_u8
            {
                output+=1
            }
        })
    });
    Some(output)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
