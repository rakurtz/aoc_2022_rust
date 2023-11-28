use std::collections::HashSet;
use std::{thread, time};

use super::super::read_file;

pub fn run() {
    let input = read_file(9).expect("Couldn't read file");

    // Part 1
    let mut rope_part1 = Rope::new(2);
    for line in input.clone().lines() {
        rope_part1.parse_and_execute_string_command(String::from(line));
    }

    let mut rope_part2 = Rope::new(10);
    for line in input.clone().lines() {
        rope_part2.parse_and_execute_string_command(String::from(line));
    }

    println!("Day 9, part 1 - {}", rope_part1.visited_by_tail.len());
    println!("Day 9, part 2 - {}", rope_part2.visited_by_tail.len());
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug, Copy, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
    visited_by_tail: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(count: u32) -> Self {
        let mut knots = Vec::new();
        for _ in 0..count {
            knots.push(Knot { x: 0, y: 0 });
        }

        let mut visited_by_tail = HashSet::new();
        visited_by_tail.insert((0, 0));

        Rope {
            knots,
            visited_by_tail,
        }
    }

    // fn print_grid(&self) {
    //     let normalizer = 0;
    //     let mut normalized_vec = Vec::new();

    //     if !self.knots.len() == 10 {
    //         panic!();
    //     }

    //     // normalize points to 0 by shifting
    //     for knot in self.knots.clone() {
    //         let nx = knot.x + normalizer;
    //         let ny = knot.y + normalizer;
    //         normalized_vec.push((nx, ny));
    //     }

    //     // print at position
    //     for (x, y) in normalized_vec.clone() {
    //         let x = x - 1;
    //         print!("{esc}[{x};{y}H", esc = 27 as char);
    //         print!("#");
    //     }

    //     let millis = time::Duration::from_millis(50);
    //     thread::sleep(millis);

    //     // send clear command
    //     print!("{}[2J", 27 as char);
    // }

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
            self.move_first_one(&command);
        }
    }

    fn move_first_one(&mut self, direction: &Direction) {
        // move head
        let first = 0;
        match direction {
            Direction::UP => self.knots[first].y += 1,
            Direction::DOWN => self.knots[first].y -= 1,
            Direction::RIGHT => self.knots[first].x += 1,
            Direction::LEFT => self.knots[first].x -= 1,
        }

        while let Some(position) = self.find_knot_to_move() {
            self.move_knot(position);
        }
    }

    fn find_knot_to_move(&self) -> Option<usize> {
        // returning the position of knot in the self.knots vector which has to move
        for position in 0..self.knots.len() - 1 {
            let next = position + 1;

            let x_diff = self.knots[position].x - self.knots[next].x;
            let y_diff = self.knots[position].y - self.knots[next].y;

            if x_diff.abs() >= 2 || y_diff.abs() >= 2 {
                return Some(next);
            }
        }
        None
    }

    fn move_knot(&mut self, position: usize) {
        let front_knot = self.knots.get(position - 1).unwrap().clone();
        let last_knot = self.knots.len() - 1; // needed to do this up here befor self.knots get borrowed :/

        let knot = &mut self.knots[position];

        let x_difference = knot.x - front_knot.x;
        let y_difference = knot.y - front_knot.y;

        if x_difference.abs() >= 2 {
            knot.y = front_knot.y; // correct in case, maybe no change though
            if front_knot.x > knot.x {
                // front_knot is on the right
                knot.x = front_knot.x - 1;
            } else {
                // front_knot is on the left
                knot.x = front_knot.x + 1;
            }
        } else if y_difference.abs() >= 2 {
            knot.x = front_knot.x; // correct in case, maybe no change though
            if front_knot.y > knot.y {
                // front_knot is on the up
                knot.y = front_knot.y - 1;
            } else {
                // front_knot is on below
                knot.y = front_knot.y + 1;
            }
        }

        // adding the new position of last knot (tail) to hashmap
        if position == last_knot {
            self.visited_by_tail.insert((knot.x, knot.y));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        input_1: String,
        input_2: String,
    }

    impl Input {
        fn new() -> Self {
            Input {
                input_1: "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
                .to_string(),
                input_2: "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                    .to_string(),
            }
        }
    }

    #[test]
    fn day_9_part1() {
        let input = Input::new();
        let mut rope = Rope::new(2);

        for line in input.input_1.lines() {
            rope.parse_and_execute_string_command(String::from(line));
            // rope.parse_and_execute_string_command(String::from(line));
        }

        let visited = rope.visited_by_tail.len();
        println!("visited: {}", visited);

        assert_eq!(13, visited);
    }

    #[test]
    fn day_9_part2() {
        let input = Input::new();
        let mut rope = Rope::new(10);

        for line in input.input_2.lines() {
            rope.parse_and_execute_string_command(String::from(line));
            // rope.parse_and_execute_string_command(String::from(line));
        }

        let visited = rope.visited_by_tail.len();
        println!("visited: {}", visited);

        assert_eq!(36, visited);
    }
}
