use super::super::read_file;

pub fn run() {
    // read file to string
    let input = read_file(4).expect("Couldn't read file");
    println!("Day 4, part 1 - {}", pt1_calculate(input.clone()));
    println!("Day 4, part 2 - {}", pt2_calculate(input.clone()));
}

struct Range {
    start: usize,
    end: usize,
    range: Vec<usize>,
}

impl Range {
    fn get_ranges_by_line(line: &str) -> (Range, Range) {
        let mut str_ranges: Vec<&str> = line.split(',').collect();
        let mut str_range_2: Vec<&str> = str_ranges.pop().unwrap().split('-').collect();
        let mut str_range_1: Vec<&str> = str_ranges.pop().unwrap().split('-').collect();

        let end = str_range_1.pop().unwrap().parse::<usize>().unwrap();
        let start = str_range_1.pop().unwrap().parse::<usize>().unwrap();
        let range = (start..=end).collect();

        let range1 = Range {
            start: start,
            end: end,
            range: range,
        };

        let end = str_range_2.pop().unwrap().parse::<usize>().unwrap();
        let start = str_range_2.pop().unwrap().parse::<usize>().unwrap();
        let range = (start..=end).collect();

        let range2 = Range {
            start: start,
            end: end,
            range: range,
        };

        (range1, range2)
    }
}

fn pt1_calculate(input: String) -> usize {
    // range is fully contained by the other
    let mut count_fully_containing = 0;

    for line in input.lines() {
        let (range1, range2) = Range::get_ranges_by_line(line);

        if (range1.range.contains(&range2.start) && range1.range.contains(&range2.end))
            || (range2.range.contains(&range1.start) && range2.range.contains(&range1.end))
        {
            count_fully_containing += 1;
        }
    }
    count_fully_containing
}

fn pt2_calculate(input: String) -> usize {
    // ranges overlap
    let mut count_overlapping = 0;

    for line in input.lines() {
        let (range1, range2) = Range::get_ranges_by_line(line);

        if range1.range.contains(&range2.start)
            || range1.range.contains(&range2.end)
            || range2.range.contains(&range1.start)
            || range2.range.contains(&range1.end)
        {
            count_overlapping += 1;
        }
    }
    count_overlapping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_4_sample_input_for_pt1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();

        assert_eq!(2, pt1_calculate(input));
    }
    #[test]
    fn day_4_sample_input_for_pt2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();

        assert_eq!(4, pt2_calculate(input));
    }
}
