use super::super::read_file;

pub fn run() {
    let input = read_file(8).expect("Couldn't read file");

    // calling with 9 piles because that is known
    println!("Day 8, part 1 - {}", calculate_part1(input));
    // println!("Day 8, part 2 - {}", calculate());
}

#[derive(Debug)]
struct Trees {
    matrix: Vec<Vec<u32>>,
    number_rows: u32,
    number_colums: u32,
    visible_trees: u32,
    trees_already_checked_for_visibility: Vec<(u32, u32)>,
    actual_tree: (u32, u32),
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

        return Trees { 
            matrix: matrix, 
            number_rows: number_of_rows, 
            number_colums: number_of_colums, 
            visible_trees: 0, 
            trees_already_checked_for_visibility: vec![], 
            actual_tree: (0, 0),
        }

    }

    fn next_tree(&mut self) -> Option<(u32, u32)> {
        let (r, c) = self.actual_tree;
        if c < self.number_colums - 1 {
            self.actual_tree = (r, c + 1 );
            Some(self.actual_tree)
        } else if r < self.number_rows - 1 {
            self.actual_tree = (r + 1, 0);
            Some(self.actual_tree)
            
        } else {
            None
        }
    }

    fn tree_height(&self, given_tree: (u32, u32)) -> u32 {
        let (r, c ) = given_tree;
        self.matrix[r as usize][c as usize]
    }


    fn check_tree_for_visibility(&mut self) -> bool {
        // checks if tree is visible from any direction 
       
        // adding tree to the trees_already_checked_for_visibility vector
        self.trees_already_checked_for_visibility.push(self.actual_tree);

        let (r, c ) = self.actual_tree;
        let own_height = self.tree_height(self.actual_tree);
  
        let mut west = true; 
        let mut east = true; 
        let mut north = true; 
        let mut south = true; 

        for west_tree in 0..c {
            let foreign_tree =  (r, west_tree);
            let foreign_height = self.tree_height(foreign_tree);
            
            if foreign_height >= own_height {
                west = false;
                break;
            } 
        }

        
        for east_tree in c+1..self.number_colums {
            let foreign_tree =  (r, east_tree);
            if self.tree_height(foreign_tree) >= own_height {
                east = false;
                break;
            } 
        }

        
        for north_tree in 0..r {
            let foreign_tree =  (north_tree, c);
            if self.tree_height(foreign_tree) >= own_height {
                north = false;
                break;
            } 
        }

        for south_tree in r+1..self.number_rows {
            let foreign_tree =  (south_tree, c);
            if self.tree_height(foreign_tree) >= own_height {
                south = false;
                break;
            } 
        }
        

        // return if any is true
        if west || east || north || south {
            true
        } else {
            false
        }

    
    }

}



fn calculate_part1(input: String) -> u32 {
    let mut trees = Trees::from_string(input);
    
    // starting with 1 because tree.actual_tree is visible (edge!)
    trees.visible_trees =  1; 
    trees.trees_already_checked_for_visibility.push(trees.actual_tree);

    loop {
        if let Some(_) = trees.next_tree() {
            if !trees.trees_already_checked_for_visibility.contains(&trees.actual_tree) {
                if trees.check_tree_for_visibility() {
                    trees.visible_trees += 1;
                }
            } 
        } else {
            break;
        }
    }

    trees.visible_trees    
}





#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        input: String,
    }

    impl Input {
        fn new() -> Self {
            Input { input: "30373
25512
65332
33549
35390".to_string() }
        }
        
    }


    #[test]
    fn day_8_p1() {
        let input = Input::new();
        assert_eq!(21, calculate_part1(input.input));
    }

    #[test]
    fn day_8_internal_next_tree() {
        let input = Input::new();
        let mut trees = Trees::from_string(input.input);

        assert_eq!(Some((0,1)), trees.next_tree());
        assert_eq!(Some((0,2)), trees.next_tree());
        assert_eq!(Some((0,3)), trees.next_tree());
        assert_eq!(Some((0,4)), trees.next_tree());
        assert_eq!(Some((1,0)), trees.next_tree());

        trees.actual_tree = (0, 1);
        assert_eq!(true, trees.check_tree_for_visibility());

        trees.actual_tree = (1, 4);
        assert_eq!(true, trees.check_tree_for_visibility());

        // muss (1, 1) sein
        trees.actual_tree = (1, 1);
        assert_eq!(true, trees.check_tree_for_visibility());
        
        trees.actual_tree = (1, 3);
        assert_eq!(false, trees.check_tree_for_visibility());
        
        trees.actual_tree = (2, 1);
        assert_eq!(true, trees.check_tree_for_visibility());

    }

    
}
