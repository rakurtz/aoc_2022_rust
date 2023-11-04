// Advent of Code 2022 in Rust
// Day 3

use std::fs;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn duplicate_item(line: &str) -> Result<char, String> {
    let compartment_a = &line[0..line.len() / 2];
    let compartnemt_b = &line[line.len() / 2..];
    for c in compartment_a.chars() {
        if compartnemt_b.contains(c) {
            return Ok(c);
        }
    }
    Err("no duplicate result found".to_string())
}

fn priority_of_char(c: char) -> usize {
    // returns the priority of a given char c by finding its position in the alphabet (static ASCII_LOWER)
    // since the priority for upper case charecters ist the same as for lower case character plus 26 
    // the .position() is only run on to_lowercase()ed characters

    if c.is_lowercase() {
        let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap();
        return index + 1;
    } else {
        let index = ASCII_LOWER
            .iter()
            .position(|&r| r == c.to_lowercase().next().unwrap())
            .unwrap();
        return index + 1 + 26;
    }
}

fn pt1_calculate(input: String) -> usize {
    let mut priority_sum = 0;

    for line in input.lines() {
        let c = duplicate_item(line).unwrap();
        priority_sum += priority_of_char(c)
    }

    priority_sum
}

// fn pt2_calculate() {
//     todo!();
// }

fn main() {
    // read file to string
    let input = fs::read_to_string("puzzle_inputs/input.txt").expect("Could not read input file");

    println!("Day 3, part 1 - {}", pt1_calculate(input.clone()));
    // println!("Day 3, part 2 - {}", pt2_calculate(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_for_pt1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(157, pt1_calculate(input));
    }
}
