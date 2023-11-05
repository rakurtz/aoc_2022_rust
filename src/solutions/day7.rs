use super::super::read_file;
use std::collections::HashMap;


pub fn run() {
    // read file to string
    let input = read_file(7).expect("Couldn't read file");
    println!("Day 7, part 1 - {}", pt1_calculate(input.clone()));
    // println!("Day 7, part 2 - {}", pt2_calculate(input.clone()));
}

fn pt1_calculate(input: String) -> usize {
    let mut root_dir = Directory {
        id: 0,
        is_read: false,
        name: "root".to_string(),
        parent_id: None,
        directories: Vec::new(),
        files: Vec::new(),
        flat_size: 0,
        deep_size: 0,
    }

    let mut state = State {
        dir_map: HashMap::new(),
        file_map: HashMap::new(),
        current_dir_id: 0,
        current_level: 0,
    };


    for line in input.lines() {
        let expr =  line.split(' ').collect().get(0).unwrap();
        match expr {
            "$" => parse_command(line),
            "dir" => state.register_directory(line),
            _   => state.current_dir.add_file(),
        }
        
    }
    todo!();
}

struct State {
    dir_map: HashMap<usize, Directory>,
    file_map: HashMap<usize, File>,
    current_dir_id: usize,
    current_level: usize,
}

impl State {

    fn level_into(&mut self) {
        self.level += 1;
    }

    fn level_out(&mut self) {
        if self.level >= 1 {
            self.level -= 1;
        } else {
            println!("Warning, already in root-level");
        }
    }

    fn register_directory(&mut self, dir: &mut Directory) -> usize { // return id
        todo!();
        // generate hash via self.generate_id(dir.name)
        // write hash into dir.id
        // add to self.dir_map
        // return id
    }

    fn generate_id(&self, name: &str) -> usize {
        todo!();
        // what kind of id (hash / random nummer??) shall we use here
        // accordingly change types in State, Directory, and File
    }

    fn current_dir(&self) -> &Directory {
        let (_, dir) = self.dir_map.get_key_value(&self.current_dir_id).unwrap();
        dir
    }

}


struct Directory {
    // todo: impl. defaults
    id: usize,  // root has id 0
    is_read: bool,
    name: String,
    parent_id: Option<usize>, // root diretory has none
    directories: Vec<usize>,
    files: Vec<usize>,
    flat_size: usize,
    deep_size: usize,
} 

impl Directory {
    fn register_new_dir(line: &str, parent_id: usize, state: &mut State) -> usize {  // returning the id of directory
        todo!();
        // generate_id and add empty dir into hash_map
        // return id
    }
    
    fn read_dir(&mut self, line: &str, state: &mut State) {
        todo!();
        // level.down()
        // call register_new_dir for each dir
        // create and add Files to self.files and add to flat_size
    }
    
    fn return_to_parent_dir(&self, parent: usize, state: &mut State) -> Directory {
        todo!();
        
        // add own flat_size to parent
        // set is_read to true
        // return hash_map.key(parent)
        
    }
    
    
}



struct File {
    id: usize,
    name: String,
    size: usize
}

struct Level {
    level: usize,
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
    7214296 k".to_string();

        assert_eq!(95437, pt1_calculate(input));
    }

}
