use super::super::read_file;

use std::collections::{HashMap, VecDeque};

pub fn run() {
    // read file to string
    let input = read_file(6).expect("Couldn't read file");
    println!(
        "Day 6, part 1 - {}",
        return_first_distinct_with_buffer_size(input.clone(), 4).unwrap()
    );
    println!(
        "Day 6, part 2 - {}",
        return_first_distinct_with_buffer_size(input.clone(), 14).unwrap()
    );
}

fn buffer_contains_four_unique_chars(buffer: &VecDeque<char>, buffer_size: usize) -> bool {
    // using a hashmap to figure out if uniqe characters
    // todo: shouldn't there be a method or at least a more concise way?
    let mut hash_map = HashMap::with_capacity(buffer_size);

    for c in buffer {
        // value is not of interest here, could use anything
        hash_map.insert(c, 0);
    }
    // returns false when there were duplicate charecters in buffer
    hash_map.len() == buffer_size
}

fn return_first_distinct_with_buffer_size(input: String, buffer_size: usize) -> Option<usize> {
    // how to use a VecDeque with a fixes size?
    let mut input_chars = input.chars();
    let mut buffer = VecDeque::new();
    let mut counter: usize;

    // fill buffer
    counter = buffer_size;
    for _ in 0..buffer_size {
        buffer.push_back(input_chars.next().unwrap());
    }

    for c in input_chars {
        // test if buffer already contains 4 unique chars
        if buffer_contains_four_unique_chars(&buffer, buffer_size) {
            return Some(counter);
        } else {
            counter += 1;
            buffer.push_back(c);

            // todo: Use VecDeque with a fixed size
            buffer.pop_front();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_6_sample_input_for_pt1() {
        let example_1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(); // after 5 chars
        let example_2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string(); // after 6 chars
        let example_3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(); // after 10 chars
        let example_4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(); // after 11 chars

        assert_eq!(Some(5), return_first_distinct_with_buffer_size(example_1, 4));
        assert_eq!(Some(6), return_first_distinct_with_buffer_size(example_2, 4));
        assert_eq!(Some(10), return_first_distinct_with_buffer_size(example_3, 4));
        assert_eq!(Some(11), return_first_distinct_with_buffer_size(example_4, 4));
    }

    #[test]
    fn day_6_sample_input_for_pt2() {
        let example_1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(); // after 19 chars
        let example_2 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(); // after 23 chars
        let example_3 = "nppdvjthqldpwncqszvftbrmjlhg".to_string(); // after 23 chars
        let example_4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(); // after 29 chars
        let example_5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(); // after 26 chars

        assert_eq!(Some(19), return_first_distinct_with_buffer_size(example_1, 14));
        assert_eq!(Some(23), return_first_distinct_with_buffer_size(example_2, 14));
        assert_eq!(Some(23), return_first_distinct_with_buffer_size(example_3, 14));
        assert_eq!(Some(29), return_first_distinct_with_buffer_size(example_4, 14));
        assert_eq!(Some(26), return_first_distinct_with_buffer_size(example_5, 14));
    }
}
