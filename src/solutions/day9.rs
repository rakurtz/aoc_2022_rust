use std::collections::HashSet;

use super::super::read_file;

pub fn run() {
    let input = read_file(9).expect("Couldn't read file");

    let mut rope = Rope::new();

    for line in input.lines() {
        rope.parse_and_execute_string_command(String::from(line));
    }

    println!("Day 9, part 1 - {}", rope.visited_by_tail.len());
    // println!("Day 9, part 1 - {}", trees.max_scenic_score);
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug)]
struct Rope {
    head: (i32, i32), // (x, y)
    tail: (i32, i32),
    visited_by_tail: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited_by_tail: HashSet::new(),
        }
    }

    fn parse_and_execute_string_command(&mut self, line: String) {
        let mut splitted = line.split_whitespace();
        let command = match splitted.next().unwrap() {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "R" => Direction::RIGHT,
            "L" => Direction::LEFT,
            _ => panic!(),
        };
        let count = splitted.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            self.move_one(&command);
        }
    }

    fn move_one(&mut self, direction: &Direction) {
        // move head
        match direction {
            Direction::UP => self.head.1 += 1,
            Direction::DOWN => self.head.1 -= 1,
            Direction::RIGHT => self.head.0 += 1,
            Direction::LEFT => self.head.0 -= 1,
        }

        // pull tail
        let x_difference = self.head.0 - self.tail.0;
        let y_difference = self.head.1 - self.tail.1;

        match direction {
            Direction::RIGHT | Direction::LEFT => {
                // head is on the right side
                if x_difference > 1 {
                    self.tail.0 = self.head.0 - 1;
                }

                // head is on the left side
                if x_difference < -1 {
                    self.tail.0 = self.head.0 + 1;
                }

                // pull tail vertically
                if y_difference.abs() + x_difference.abs() >= 3 {
                    self.tail.1 = self.head.1;
                }
            }

            Direction::UP | Direction::DOWN => {
                // head is on the upper side
                if y_difference > 1 {
                    self.tail.1 = self.head.1 - 1;
                }

                // head is on the left side
                if y_difference < -1 {
                    self.tail.1 = self.head.1 + 1;
                }

                // pull tail horizontally
                if y_difference.abs() + x_difference.abs() >= 3 {
                    self.tail.0 = self.head.0;
                }
            }
        }

        // // Debug Print
        // println!(
        //     "{:?}: Head: {:?}, Tail: {:?}",
        //     direction, self.head, self.tail
        // );

        self.visited_by_tail.insert(self.tail.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        input: String,
    }

    impl Input {
        fn new() -> Self {
            Input {
                input: "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
                .to_string(),
            }
        }
    }

    #[test]
    fn day_9_test_results() {
        let input = Input::new();
        let mut rope = Rope::new();

        for line in input.input.lines() {
            rope.parse_and_execute_string_command(String::from(line));
        }

        let visited = rope.visited_by_tail.len();
        println!("visited: {}", visited);

        assert_eq!(13, visited);
    }
}
