// Advent of Code 2022 in Rust
// Day 2

use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum GameAction {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn match_result_by_str(s: &str) -> GameResult {
        match s {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => {
                panic!("could not parse input")
            }
        }
    }

    fn match_action_to_achieve(&self, opponent: &GameAction) -> GameAction {
        if *self == GameResult::Draw {
            *opponent
        } else if *self == GameResult::Win {
            match *opponent {
                GameAction::Rock => GameAction::Paper,
                GameAction::Paper => GameAction::Scissor,
                GameAction::Scissor => GameAction::Rock,
            }
        } else {
            // lose
            match *opponent {
                GameAction::Rock => GameAction::Scissor,
                GameAction::Paper => GameAction::Rock,
                GameAction::Scissor => GameAction::Paper,
            }
        }
    }
}

impl GameAction {
    fn action_score(&self) -> u32 {
        // the score of players choice
        match *self {
            GameAction::Rock => 1,
            GameAction::Paper => 2,
            GameAction::Scissor => 3,
        }
    }

    fn match_action_by_str(s: &str) -> GameAction {
        match s {
            "A" | "X" => GameAction::Rock,
            "B" | "Y" => GameAction::Paper,
            "C" | "Z" => GameAction::Scissor,
            _ => {
                panic!("could not parse input")
            }
        }
    }

    fn game_score(&self, oppenent: &GameAction) -> u32 {
        // the game's score
        if self == oppenent {
            // draw
            3
        } else if (*self == GameAction::Rock && *oppenent == GameAction::Scissor)
            || (*self == GameAction::Paper && *oppenent == GameAction::Rock)
            || (*self == GameAction::Scissor && *oppenent == GameAction::Paper)
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

        let opponent = GameAction::match_action_by_str(v.get(0).unwrap());
        let player = GameAction::match_action_by_str(v.get(1).unwrap());

        score += player.action_score();
        score += player.game_score(&opponent);
    }
    score
}

fn pt2_calculate_score(input: String) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let v: Vec<&str> = line.split(' ').collect();

        let opponent = GameAction::match_action_by_str(v.get(0).unwrap());
        let game_result = GameResult::match_result_by_str(v.get(1).unwrap());
        let player = game_result.match_action_to_achieve(&opponent);

        score += player.action_score();
        score += player.game_score(&opponent);
    }
    score
}

fn main() {
    // read file to string
    let input = fs::read_to_string("puzzle_inputs/input.txt").expect("Could not read input file");

    println!("Day 2, part 1 - {}", pt1_calculate_score(input.clone()));
    println!("Day 2, part 2 - {}", pt2_calculate_score(input.clone()));
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

    #[test]
    fn sample_input_for_pt2() {
        let input = "A Y
B X
C Z"
        .to_string();
        assert_eq!(12, pt2_calculate_score(input));
    }
}
