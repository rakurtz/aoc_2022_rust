use super::super::read_file;

pub fn run() {
    // read file to string
    let input = read_file(7).expect("Couldn't read file");
    println!("Day 7, part 1 - {}", pt1_calculate(input.clone()));
    // println!("Day 7, part 2 - {}", pt2_calculate(input.clone()));
}

fn pt1_calculate(input: String) -> String{
    todo!();
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_7_sample_input_for_pt1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();

        assert_eq!("Ok", pt1_calculate(input));
    }

}
