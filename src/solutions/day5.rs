use super::super::read_file;
use std::collections::VecDeque;

pub fn run() {
    let input = read_file(5).expect("Couldn't read file");

    // calling with 9 piles because that is known
    println!("Day 5, part 1 - {}", calculate(input.clone(), 9, 1));
    println!("Day 5, part 2 - {}", calculate(input.clone(), 9, 2));;
}

#[derive(Debug)]
struct Storage {
    storage: Vec<VecDeque<char>>,
}

impl Storage {
    fn new_with_amount_of_piles(piles: usize) -> Self {
        let mut storage = Storage {
            storage: Vec::new(),
        };

        for _ in 0..piles {
            storage.storage.push(VecDeque::new());
        }
        storage
    }

    fn poulate_via_storage_pattern(&mut self, storage_pattern: String) {
        for line in storage_pattern.lines() {
            let mut chars = line.chars();
            let (_, len) = chars.size_hint();

            for pos in 0..=len.unwrap() / 4 {
                // use chars.nth(1) to skip the first char '[' and get crate's label
                if let Some(crate_label) = chars.nth(1) {
                    if crate_label != ' ' {
                        self.storage[pos].push_front(crate_label);
                    }
                }
                // "move forward" in chars to make the above chars.nth(1) hit the correct position
                chars.nth(1);
            }
        }
    }

    fn move_crates_by_command_pattern_single_crates(&mut self, command_pattern: String) {
        for line in command_pattern.lines() {
            let v: Vec<&str> = line.split(' ').collect();

            let amount = v.get(1).unwrap().parse::<usize>().unwrap();

            // substract 1 to map 1-9 to corresponding 0-8 of storage vec
            let from_pile = v.get(3).unwrap().parse::<usize>().unwrap() - 1;
            let to_pile = v.get(5).unwrap().parse::<usize>().unwrap() - 1;

            for _ in 0..amount {
                let retrieved = self.storage[from_pile].pop_back().unwrap();
                self.storage[to_pile].push_back(retrieved);
            }
        }
    }

    fn move_crates_by_command_pattern_multi_crates(&mut self, command_pattern: String) {
        // for day 5 part 2:
        // here multiple crates shall be moved from one pile to another at once
        // so their order is preserved
        // using a temporary vec here to accomplish that

        for line in command_pattern.lines() {
            let v: Vec<&str> = line.split(' ').collect();

            let amount = v.get(1).unwrap().parse::<usize>().unwrap();

            // substract 1 to map 1-9 to corresponding 0-8 of storage vec
            let from_pile = v.get(3).unwrap().parse::<usize>().unwrap() - 1;
            let to_pile = v.get(5).unwrap().parse::<usize>().unwrap() - 1;

            let mut temp_vec = Vec::new();
            for _ in 0..amount {
                let retrieved = self.storage[from_pile].pop_back().unwrap();
                temp_vec.push(retrieved);
            }
            for _ in 0..amount {
                self.storage[to_pile].push_back(temp_vec.pop().unwrap());
            } 
        }
    }

    fn top_crates_as_string(&self) -> String {
        let mut crates_string = String::new();
        for pile in &self.storage {
            // using .back() instead of pop_back to not mutate the storage
            crates_string = format!("{}{}", crates_string, pile.back().unwrap());
        }
        crates_string
    }
}

fn calculate(input: String, amount_of_piles: usize, part: u8) -> String {
    // amount of piles differs for tests (3) and for the puzzle input (9)
    // last functions argument (part 1 or 2) defines which puzzle solution to be run
    
    let mut storage = Storage::new_with_amount_of_piles(amount_of_piles);

    // split input into storage_pattern and command_pattern
    let mut storage_pattern = String::new();
    let mut command_pattern = String::new();
    for line in input.lines() {
        // using "[" and "move" to differenciate two input_sets
        if line.contains("[") {
            storage_pattern = format!("{}{}\n", storage_pattern, line);
        } else if line.contains("move") {
            command_pattern = format!("{}{}\n", command_pattern, line);
        }
    }

    // poulate the storage
    storage.poulate_via_storage_pattern(storage_pattern);
    match part {
        1 => storage.move_crates_by_command_pattern_single_crates(command_pattern),
        2 => storage.move_crates_by_command_pattern_multi_crates(command_pattern),
        _ => panic!("fn calculate was called without part specified"),
    }
    

    // retrieve and return top crate
    storage.top_crates_as_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_sample_input_for_pt1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();

        // calling with 3 piles for test input because it is known...
        assert_eq!("CMZ", calculate(input, 3, 1));
    }


    #[test]
    fn day_5_sample_input_for_pt2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();

        // calling with 3 piles for test input because it is known...
        assert_eq!("MCD", calculate(input, 3, 2));
    }
}
