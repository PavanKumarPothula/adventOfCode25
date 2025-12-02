advent_of_code::solution!(1);
const DIAL_SIZE: i64 = 100;
const START_POS: i64 = 50;

pub fn input_as_i64_slice(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| {
            x.replace("R", "")
                .replace("L", "-")
                .parse::<i64>()
                .expect("Something is off")
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let rot_slice = input_as_i64_slice(input);
    let mut current_pos: i64 = START_POS;
    let mut next_pos: i64;
    let mut out: i64 = 0;
    for rot in rot_slice {
        if rot == 0 {
            continue;
        }
        next_pos = (current_pos + rot) % DIAL_SIZE;
        if next_pos == 0 {
            out += 1;
        }
        current_pos = next_pos;
    }
    return Some(out);
}

pub fn part_two(input: &str) -> Option<i64> {
    // Answer too HIGH?!

    // Outline
    //
    // (START_POS) = 50
    // ==========================================
    // + rot = intr/DIAL = curCrossing :: next => out
    // ==========================================
    // +  55 =  105/100 =       1  :: 5    => 1
    // + 100 =  105/100 =       1  :: 5    => 2
    // + 250 =  255/100 =       2  :: 55   => 4
    // +  45 =  100/100 =       1  :: 0    => 5
    // -  55 =  -10/100 =       1  :: 90   => 6
    // - 210 = -120/100 =       2  :: 80   => 8
    // -  80 =    0/100 =       1  :: 0    => 9
    // + 200 =  200/100 =       2  :: 0    => 11
    // - 200 = -200/100 =       2  :: 0    => 13
    // +  45 =   45/100 =       0  :: 45   => 13
    // + 445 =  490/100 =       4  :: 90   => 17
    // - 990 = -900/100 =      10  :: 0    => 27
    // +  45 =   45/100 =       0  :: 45   => 27
    // - 445 = -400/100 =       4  :: 90   => 17
    // - 160 =  -50/100 =       1  :: 50   =>  18
    // -  51 =   -1/100 =       1  :: 99   =>  19

    let rot_slice = input_as_i64_slice(input);
    let mut current_position: i64 = START_POS;
    let mut total_zero_crossings: i64 = 0;
    let mut current_zero_crossing: i64;
    for rotation in rot_slice {
        if rotation == 0 {
            // dont waste cycles on this
            continue;
        }

        // Count just the full circle first. i.e. /100
        current_zero_crossing = ((rotation + current_position).abs()) / DIAL_SIZE;

        // Special cases for zero crossing in counter-clockwise
        if (current_position != 0) && (current_position <= (-rotation)) {
            current_zero_crossing = current_zero_crossing + 1;
        }

        total_zero_crossings = total_zero_crossings + current_zero_crossing;

        current_position = (current_position + rotation).rem_euclid(DIAL_SIZE);
    }
    return Some(total_zero_crossings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
