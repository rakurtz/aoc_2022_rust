// Advent of Code 2022 in Rust
// Day 2

use std::fs;

// question here: Why do in need to derive PartialEQ
// when i want to compare the very types/values itself in fn game_score?
#[derive(PartialEq)]
enum RPC {
    Rock,
    Paper,
    Scissor,
}

impl RPC {
    fn rpc_score(&self) -> u32 {
        // the score of players choice
        match *self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissor => 3,
        }
    }
    fn game_score(&self, oppenent: &RPC) -> u32 {
        // the game's score
        // question: in line 27 it seems to be ok to not *self and *opponent. 
        // when i remove the * in lines 31,32,33, it wouldn't compile with message: 
        // "no implementation for `&RPC == RPC`"
        if self == oppenent {
            // draw
            3
        } else if (*self == RPC::Rock && *oppenent == RPC::Scissor)
            || (*self == RPC::Paper && *oppenent == RPC::Rock)
            || (*self == RPC::Scissor && *oppenent == RPC::Paper)
        {
            // winning
            6
        } else {
            // losing...
            0
        }
    }
}

fn pt1_calculate_score(input: String) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split(' ').collect();

        // style: should the matching be refactored to a seperate function?
        let opponent = match v[0] {
            "A" => RPC::Rock,
            "B" => RPC::Paper,
            "C" => RPC::Scissor,
            _ => {
                panic!("could parse input")
            }
        };
        let player = match v[1] {
            "X" => RPC::Rock,
            "Y" => RPC::Paper,
            "Z" => RPC::Scissor,
            _ => {
                panic!("could parse input")
            }
        };

        score += player.rpc_score();
        score += player.game_score(&opponent);
    }
    score
}

fn main() {
    // read file to string
    let input = fs::read_to_string("puzzle_inputs/input.txt").expect("Could not read input file");

    println!("Day 2, part 1 - {}", pt1_calculate_score(input));
    // println!("Day 2, part 2 - {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_for_pt1() {
        let input = "A Y
B X
C Z"
        .to_string();
        assert_eq!(15, pt1_calculate_score(input));
    }
}
