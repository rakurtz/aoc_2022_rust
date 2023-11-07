use super::super::read_file;
use std::collections::HashMap;

// given by puzzle instructions
const FILESYSTEM_TOTAL: usize = 70000000;
const FILESYSTEM_NEEDED: usize = 30000000;

pub fn run() {
    // read file to string
    let input = read_file(7).expect("Couldn't read file");
    let mut file_system = generate_filesystem_based_on_input(input.clone());

    println!(
        "Day 7, part 1 - {}",
        file_system.sum_of_size_at_most_100000()
    );
    println!(
        "Day 7, part 2 - {}",
        file_system.size_of_smallest_dir_to_delete_to_free_needed()
    );
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

    fn get_mut_parent_dir(&mut self) -> &mut Directory {
        let parent_id = self.get_mut_current_dir().parent_id.unwrap();
        let dir = self.dir_map.get_mut(&parent_id).unwrap();
        dir
    }

    fn get_mut_current_dir(&mut self) -> &mut Directory {
        let dir = self.dir_map.get_mut(&self.current_dir_id).unwrap();
        dir
    }

    fn get_root_dir(&mut self) -> &Directory {
        let dir = self.dir_map.get(&0).unwrap();
        dir
    }

    fn parse_command(&mut self, line: &str) {
        if line.contains("cd /") {
            self.current_dir_id = 0;
        } else if line.contains("cd ..") {
            //add current size to parent size
            self.get_mut_parent_dir().size += self.get_mut_current_dir().size;
            self.current_dir_id = self.get_mut_current_dir().parent_id.unwrap();
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

    fn size_of_smallest_dir_to_delete_to_free_needed(&mut self) -> usize {
        let used = self.get_root_dir().size;
        dbg!(used);
        let needed_space = FILESYSTEM_NEEDED - (FILESYSTEM_TOTAL - used);

        let mut sufficient_dirs: Vec<usize> = Vec::new();
        for (_, dir) in &self.dir_map {
            if dir.size >= needed_space {
                sufficient_dirs.push(dir.size);
            }
        }
        *sufficient_dirs.iter().min().unwrap()
    }
}

fn generate_filesystem_based_on_input(input: String) -> FileSystem {
    // generating our starting structs
    let root_dir = Directory {
        id: 0,
        parent_id: None,
        //directories: Vec::new(),
        size: 0,
    };

    let mut files_system = FileSystem {
        dir_map: HashMap::new(),
        current_dir_id: root_dir.id,
        counter: 0,
    };

    files_system.dir_map.insert(0, root_dir);

    // parsing each line
    for line in input.lines() {
        files_system.counter += 1;
        let splitted: Vec<&str> = line.split(' ').collect();

        if let Some(expr) = splitted.get(0) {
            match *expr {
                "$" => {
                    files_system.parse_command(line);
                }
                "dir" => (),

                // all other values will be files with their file-size as first value
                _ => {
                    let size = expr.parse::<usize>().unwrap();
                    files_system.get_mut_current_dir().size += size;
                }
            }
        }
    }

    // fixed above unsufficent logic: after parsing the last line, the size of the current
    // dir isn`t added to the above dirs. This could be at any level. So we need to "cd .." up
    // to the root dir (with id 0) while adding the sizes (which is done within the parse_command function)
    while files_system.current_dir_id > 0 {
        files_system.parse_command("cd ..")
    }

    // returning the FileSystem here
    files_system
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_7_pt1_test() {
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
        let file_system = generate_filesystem_based_on_input(input);
        assert_eq!(95437, file_system.sum_of_size_at_most_100000());
    }

    #[test]
    fn day_7_pt2_test() {
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
        let mut file_system = generate_filesystem_based_on_input(input);
        assert_eq!(
            24933642,
            file_system.size_of_smallest_dir_to_delete_to_free_needed()
        );
    }
}
