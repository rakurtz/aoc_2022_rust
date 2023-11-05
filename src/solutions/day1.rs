use super::super::read_file;

pub fn run() {
    let mut elves = Elves::new();
    // // read file and split by empty lines, parse string to u32
    // let collection = fs::read_to_string("puzzle_inputs/day01_1.txt")
    //     .expect("Could not read file");

    let collection = read_file(1).expect("Couldn't read file");

    elves.populate_elves_from_string(collection);

    println!(
        "Day 1, part 1 - max calories by single elve carried: {}",
        elves.get_max_cal_carried_by_elve()
    );
    println!(
        "Day 1, part 2 - max calories by three elves carried: {}",
        elves.get_cal_carried_by_max_three_elve()
    );
}

struct Elve {
    calories: u32,
}

struct Elves {
    elves: Vec<Elve>,
}

impl Elves {
    fn new() -> Elves {
        Elves { elves: Vec::new() }
    }

    fn add_elve_from_vec_of_u32(&mut self, vec_of_u32: Vec<u32>) {
        self.elves.push(Elve {
            calories: vec_of_u32.iter().sum(),
        });
    }

    fn populate_elves_from_string(&mut self, collection: String) {
        // here the whole string from fs::read_to_string gets computed
        // todo: refactor to add -> Result<T, E>

        let mut cache = Vec::new();
        for line in collection.lines() {
            if line == "" {
                self.add_elve_from_vec_of_u32(cache.clone());
                cache.clear();
                continue;
            }
            cache.push(line.parse::<u32>().unwrap());
        }
        self.add_elve_from_vec_of_u32(cache.clone());
    }

    fn get_max_cal_carried_by_elve(&self) -> u32 {
        // this answers part 1
        // implemented with a for loop and a cache variable (max_calories)

        let mut max_calories = 0;
        for elve in &self.elves {
            if elve.calories > max_calories {
                max_calories = elve.calories;
            }
        }
        max_calories
    }

    fn get_cal_carried_by_max_three_elve(&mut self) -> u32 {
        // this answerts part 2
        // impl with via sorting the elves vec.

        self.elves.sort_by(|a, b| b.calories.cmp(&a.calories));

        // todo: is there a better way to add the three top elves.calories?
        // wanted to to with reduce but the vec consists of type Elve and i need u32 values
        let mut calories = 0;
        for elve in self.elves.get(0..3).unwrap() {
            calories += elve.calories;
        }
        calories
    }

    // fn print_elves(&self) {
    //     // todo: substitue with impl Display for Elve and Elves
    //     for elve in &self.elves {
    //         println!("{}", elve.calories)
    //     }
    // }
}
