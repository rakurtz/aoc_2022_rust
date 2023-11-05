use super::super::read_file;
use std::collections::HashMap;

pub fn run() {
    // read file to string
    let input = read_file(7).expect("Couldn't read file");
    println!("Day 7, part 1 - {}", pt1_calculate(input.clone()));
    // println!("Day 7, part 2 - {}", pt2_calculate(input.clone()));
}

struct Directory {
    // todo: impl. defaults
    id: usize,                // root has id 0
    parent_id: Option<usize>, // root diretory has none
    //directories: Vec<usize>,
    size: usize,
}

struct FileSystem {
    dir_map: HashMap<usize, Directory>,
    current_dir_id: usize,
    counter: usize,
}

impl FileSystem {
    fn register_and_enter_new_dir(&mut self) {
        let dir = Directory {
            id: self.counter,
            parent_id: Some(self.current_dir_id),
            //directories: Vec::new(),
            size: 0,
        };

        self.dir_map.insert(self.counter, dir);
        self.current_dir_id = self.counter;
    }

    fn get_parent_dir(&mut self) -> &mut Directory {
        let parent_id = self.get_current_dir().parent_id.unwrap();
        let dir = self.dir_map.get_mut(&parent_id).unwrap();
        dir
    }

    fn get_current_dir(&mut self) -> &mut Directory {
        let dir = self.dir_map.get_mut(&self.current_dir_id).unwrap();
        dir
    }

    fn parse_command(&mut self, line: &str) {
        if line.contains("cd /") {
            self.current_dir_id = 0;
        } else if line.contains("cd ..") {
            //add current size to parent size
            self.get_parent_dir().size += self.get_current_dir().size;
            self.current_dir_id = self.get_current_dir().parent_id.unwrap();
        } else if line.contains("cd") {
            self.register_and_enter_new_dir();
        }
    }

    fn sum_of_size_at_most_100000(&self) -> usize {
        let mut under_100000: Vec<usize> = Vec::new();
        for (_, dir) in &self.dir_map {
            if dir.size <= 100000 {
                under_100000.push(dir.size);
            }
        }
        let sum = under_100000.iter().sum();
        sum
    }
}

fn pt1_calculate(input: String) -> usize {
    // generating our starting structs
    let root_dir = Directory {
        id: 0,
        parent_id: None,
        //directories: Vec::new(),
        size: 0,
    };

    let mut state = FileSystem {
        dir_map: HashMap::new(),
        current_dir_id: root_dir.id,
        counter: 0,
    };

    state.dir_map.insert(0, root_dir);

    // parsing each line
    for line in input.lines() {
        state.counter += 1;
        let splitted: Vec<&str> = line.split(' ').collect();

        if let Some(expr) = splitted.get(0) {
            match *expr {
                "$" => {
                    state.parse_command(line);
                }
                "dir" => (),
                _ => {
                    let size = expr.parse::<usize>().unwrap();
                    state.get_current_dir().size += size;
                }
            }
        }
    }

    state.sum_of_size_at_most_100000()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_7_sample_input_for_pt1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .to_string();

        assert_eq!(95437, pt1_calculate(input));
    }
}
