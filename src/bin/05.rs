use std::ops::RangeInclusive;

advent_of_code::solution!(5);

#[derive(Default, Debug)]
struct Database {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Database {
    fn form_db(input: &str) -> Database {
        let mut db = Database::default();
        //     println!("hey!");
        input.lines().for_each(|line| {
            if !line.is_empty() {
                let x = line.split_once("-");
                match x {
                    Some((_start, _end)) => {
                        db.fresh_ranges
                            .push(_start.parse().expect("OhOh!")..=_end.parse().expect("OhOh!"));
                    }
                    None => {
                        db.ingredients.push(line.parse().expect("OhOh!"));
                    }
                }
            }
        });
        db
    }

    fn get_fresh_list(&self) -> usize {
        self.ingredients.iter().fold(0, |acc, ing| {
            if self.fresh_ranges.iter().any(|range| range.contains(ing)) {
                //             println!("{ing}");
                acc + 1
            } else {
                acc
            }
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let db = Database::form_db(input);
    // println!("{:#?}", db);
    Some(db.get_fresh_list() as u64)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
