use grid::*;

struct RollsReturn {
    new_matrix: Grid<u8>,
    output: u64,
}
advent_of_code::solution!(4);
fn rolls_to_remove(matrix: &mut Grid<u8>) -> Option<RollsReturn> {
    let mut co_ordinates_to_remove: Vec<(usize, usize)> = vec![];
    matrix
        .indexed_iter()
        .for_each(|((row_index, column_index), digit)| {
            if *digit == 1
                && (row_index > 1 && row_index < (matrix.size().0 - 2))
                && (column_index > 1 && column_index < (matrix.size().1 - 2))
                && (matrix.get(row_index - 1, column_index).unwrap_or(&0_u8)
                    + matrix.get(row_index + 1, column_index).unwrap_or(&0_u8)
                    + matrix.get(row_index - 1, column_index - 1).unwrap_or(&0_u8)
                    + matrix.get(row_index - 1, column_index + 1).unwrap_or(&0_u8)
                    + matrix.get(row_index + 1, column_index - 1).unwrap_or(&0_u8)
                    + matrix.get(row_index + 1, column_index + 1).unwrap_or(&0_u8)
                    + matrix.get(row_index, column_index - 1).unwrap_or(&0_u8)
                    + matrix.get(row_index, column_index + 1).unwrap_or(&0_u8))
                    < 4_u8
            {
                co_ordinates_to_remove.push((row_index, column_index));
            }
        });
    if co_ordinates_to_remove.len() > 0 {
        //     println!("{:#?}", matrix);
        //     println!("{:?}", co_ordinates_to_remove);
        co_ordinates_to_remove.iter().for_each(|(r, c)| {
            matrix[(*r, *c)] = 0;
        });
        Some(RollsReturn {
            output: co_ordinates_to_remove.len() as u64,
            new_matrix: matrix.clone(),
        })
    } else {
        None
    }
}
fn parse_input(input: &str) -> Grid<u8> {
    let mut matrix = Grid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| if char == '@' { 1_u8 } else { 0_u8 })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>(),
    );
    matrix.prepend_cols(2);
    matrix.prepend_rows(2);
    matrix.expand_cols(2);
    matrix.expand_rows(2);
    matrix
}

#[rustfmt::skip]
pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix = parse_input(input);
    let mut output = 0_u64;
    while let Some(current_out)= rolls_to_remove(&mut matrix){
        matrix = current_out.new_matrix;
        output += current_out.output;
    }
    Some(output)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut matrix = parse_input(input);
    // println!("looping!");
    return Some(rolls_to_remove(&mut matrix).expect("SomethingIsOff").output);
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
        assert_eq!(result, Some(43));
    }
}
