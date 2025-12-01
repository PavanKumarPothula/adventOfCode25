advent_of_code::solution!(1);
const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

pub fn input_as_i32_slice(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| {
            x.replace("R", "")
                .replace("L", "-")
                .parse::<i32>()
                .expect("Something is off")
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rot_slice = input_as_i32_slice(input);
    let mut current_pos: i32 = START_POS;
    let mut next_pos: i32;
    let mut out: u64 = 0;
    for rot in rot_slice {
        next_pos = (current_pos + rot) % DIAL_SIZE;
        if next_pos == 0 {
            out += 1;
        }
        current_pos = next_pos;
    }
    return Some(out);
}

pub fn part_two(input: &str) -> Option<u64> {
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
    let rot_slice = input_as_i32_slice(input);
    let mut current_pos: i32 = START_POS;
    let mut next_pos: i32;
    let mut out: u64 = 0;
    for rot in rot_slice {
        next_pos = current_pos + rot;
        if next_pos < 0 {
            out = out + 1;
        }
        out = out + ((next_pos / DIAL_SIZE).abs() as u64);
        current_pos = next_pos.rem_euclid(DIAL_SIZE);
    }
    return Some(out);
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
