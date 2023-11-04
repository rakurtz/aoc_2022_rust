// Advent of Code 2022 in Rust
// Day 4

use std::fs;

fn pt1_calculate(input: String) -> usize {
    let mut count_fully_containing = 0;

    for line in input.lines() {
        let line_parts:Vec<&str> = line.split(',').collect();
        let first_a_b:Vec<&str> = line_parts[0].split('-').collect();
        let second_a_b:Vec<&str> = line_parts[1].split('-').collect();
        if second_a_b[0].parse::<usize>().unwrap() >= first_a_b[0].parse().unwrap() &&
            second_a_b[1].parse::<usize>().unwrap() <= first_a_b[1].parse().unwrap() {
                count_fully_containing += 1;
            } else if second_a_b[0].parse::<usize>().unwrap() <= first_a_b[0].parse().unwrap() &&
            second_a_b[1].parse::<usize>().unwrap() >= first_a_b[1].parse().unwrap() {
                count_fully_containing += 1;
            }
    }
    count_fully_containing
}

fn main() {
    // read file to string
    let input = fs::read_to_string("puzzle_inputs/input.txt").expect("Could not read input file");

    println!("Day 4, part 1 - {}", pt1_calculate(input.clone()));
    // println!("Day 4, part 2 - {}", pt2_calculate(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_for_pt1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();

        assert_eq!(2, pt1_calculate(input));
    }
//     #[test]
//     fn sample_input_for_pt2() {
//         let input = "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw"
//             .to_string();

//         assert_eq!(70, pt2_calculate(input));
//     }
}
