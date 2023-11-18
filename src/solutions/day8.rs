use super::super::read_file;

pub fn run() {
    let input = read_file(8).expect("Couldn't read file");

    let (visible_trees, max_scenic_score) = calculate(input);
    println!("Day 8, part 1 - {}", visible_trees);
    println!("Day 8, part 1 - {}", max_scenic_score);

    // println!("Day 8, part 2 - {}", calculate());
}

#[derive(Debug)]
struct Trees {
    map_of_trees: Vec<Vec<u32>>,
    map_rows_size: u32,
    map_colums_size: u32,
    total_visible_trees: u32,
    trees_already_checked: Vec<(u32, u32)>,
    current_tree: Option<(u32, u32)>,
    max_scenic_score: u32, // row, col, score
}

impl Trees {
    fn from_string(input: String) -> Self {
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        let input_as_vec_of_lines: Vec<&str> = input.lines().collect();

        for line in input_as_vec_of_lines {
            let mut tree_column: Vec<u32> = Vec::new();

            for tree in line.chars() {
                tree_column.push(tree.to_digit(10).unwrap());
            }
            matrix.push(tree_column);
        }

        let number_of_rows = matrix.len() as u32;
        let number_of_colums = matrix[0].len() as u32;

        Trees {
            map_of_trees: matrix,
            map_rows_size: number_of_rows,
            map_colums_size: number_of_colums,
            total_visible_trees: 0,
            trees_already_checked: vec![],
            current_tree: None,
            max_scenic_score: 0,
        }
    }

    fn next_tree(&mut self) -> Option<(u32, u32)> {
        // iterate through all trees from per row from left to right 
        
        // updating self.current_tree to next tree and then return self.current_tree
        if let Some((r, c)) = self.current_tree {
            if c < self.map_colums_size - 1 {
                self.current_tree = Some((r, c + 1));
            } else if r < self.map_rows_size - 1 {
                self.current_tree = Some((r + 1, 0));
            } else {
                return None;
            }
        } else {
            self.current_tree = Some((0, 0));
        }
        self.current_tree
    }

    fn height_of_tree(&self, tree: (u32, u32)) -> u32 {
        let (r, c) = tree;
        self.map_of_trees[r as usize][c as usize]
    }

    fn visibility_and_score_of_actual_tree(&self) -> (bool, u32) {
        // checks if tree is visible from any direction, calculating score and returning both

        let (r, c) = self.current_tree.unwrap();
        let own_height = self.height_of_tree(self.current_tree.unwrap());

        let mut west_visibility = true;
        let mut east_visibility = true;
        let mut north_visibility = true;
        let mut south_visibility = true;

        let mut west_visible_trees: u32 = 0;
        let mut east_visible_trees: u32 = 0;
        let mut north_visbile_trees: u32 = 0;
        let mut south_visible_trees: u32 = 0;

        // iterating reversed here!
        for west_tree in (0..c).rev() {
            let foreign_tree = (r, west_tree);
            let foreign_height = self.height_of_tree(foreign_tree);

            if foreign_height >= own_height {
                west_visible_trees = c - west_tree;
                west_visibility = false;
                break;
            } else {
                west_visible_trees = c;
            }
        }

        for east_tree in c + 1..self.map_colums_size {
            let foreign_tree = (r, east_tree);
            if self.height_of_tree(foreign_tree) >= own_height {
                east_visible_trees = east_tree - c;
                east_visibility = false;
                break;
            } else {
                east_visible_trees = self.map_colums_size - 1 - c;
            }
        }

        // iterating reversed here!
        for north_tree in (0..r).rev() {
            let foreign_tree = (north_tree, c);
            if self.height_of_tree(foreign_tree) >= own_height {
                north_visbile_trees = r - north_tree;
                north_visibility = false;
                break;
            } else {
                north_visbile_trees = r;
            }
        }

        for south_tree in r + 1..self.map_rows_size {
            let foreign_tree = (south_tree, c);
            if self.height_of_tree(foreign_tree) >= own_height {
                south_visible_trees = south_tree - r;
                south_visibility = false;
                break;
            } else {
                south_visible_trees = self.map_rows_size - 1 - r;
            }
        }

        // results
        let visibility = west_visibility || east_visibility || north_visibility || south_visibility;
        let score = west_visible_trees * east_visible_trees * north_visbile_trees * south_visible_trees;
        (visibility, score)
    }
}

fn calculate(input: String) -> (u32, u32) {
    let mut trees = Trees::from_string(input);

    while trees.next_tree().is_some() {
        if !trees.trees_already_checked.contains(&trees.current_tree.unwrap()) {
            let (visibility, score) = trees.visibility_and_score_of_actual_tree();
            if visibility {
                trees.total_visible_trees += 1;
            }
            if score > trees.max_scenic_score {
                trees.max_scenic_score = score;
            }

            // adding tree to the trees_already_checked_for_visibility vector
            trees.trees_already_checked.push(trees.current_tree.unwrap());
        }
        
    }

    (trees.total_visible_trees, trees.max_scenic_score)
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
                input: "30373
25512
65332
33549
35390"
                    .to_string(),
            }
        }
    }

    #[test]
    fn day_8_test_results() {
        let input = Input::new();
        assert_eq!(21, calculate(input.input.clone()).0);
        assert_eq!(8, calculate(input.input.clone()).1);
    }

    #[test]
    fn day_8_internal_next() {
        let input = Input::new();
        let mut trees = Trees::from_string(input.input);

        assert_eq!(Some((0, 0)), trees.next_tree());
        assert_eq!(Some((0, 1)), trees.next_tree());
        assert_eq!(Some((0, 2)), trees.next_tree());
        assert_eq!(Some((0, 3)), trees.next_tree());
        assert_eq!(Some((0, 4)), trees.next_tree());
        assert_eq!(Some((1, 0)), trees.next_tree());
    }

    #[test]
    fn day_8_internal_visibility() {
        let input = Input::new();
        let mut trees = Trees::from_string(input.input);

        trees.current_tree = Some((0, 1));
        assert_eq!(true, trees.visibility_and_score_of_actual_tree().0);

        trees.current_tree = Some((1, 4));
        assert_eq!(true, trees.visibility_and_score_of_actual_tree().0);

        trees.current_tree = Some((1, 1));
        assert_eq!(true, trees.visibility_and_score_of_actual_tree().0);
        trees.current_tree = Some((1, 3));
        assert_eq!(false, trees.visibility_and_score_of_actual_tree().0);

        trees.current_tree = Some((2, 1));
        assert_eq!(true, trees.visibility_and_score_of_actual_tree().0);
    }

    #[test]
    fn day_8_internal_score() {
        let input = Input::new();
        let mut trees = Trees::from_string(input.input);

        trees.current_tree = Some((1, 2));
        assert_eq!(4, trees.visibility_and_score_of_actual_tree().1);

        trees.current_tree = Some((3, 2));
        assert_eq!(8, trees.visibility_and_score_of_actual_tree().1);

        trees.current_tree = Some((2, 4));
        assert_eq!(0, trees.visibility_and_score_of_actual_tree().1);

        trees.current_tree = Some((4, 4));
        assert_eq!(0, trees.visibility_and_score_of_actual_tree().1);

        trees.current_tree = Some((0, 0));
        assert_eq!(0, trees.visibility_and_score_of_actual_tree().1);
    }
}
