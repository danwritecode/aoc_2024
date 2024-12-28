use std::fs::read_to_string;

use regex::Regex;

use crate::{AdventDay, AdventReturn};


pub struct DayThree {
    file_contents: String
}

impl AdventDay for DayThree {
    fn new(input_path: &str) -> Self {
        let file_contents = read_to_string(input_path).expect("Could not read file contents");
        DayThree { file_contents } 
    }

    fn process_pt1(&self) -> AdventReturn {

        todo!()
    }

    fn process_pt2(&self) -> AdventReturn {

        todo!()
    }
}

impl DayThree {
    fn parse(&self) -> (Vec<i32>, Vec<i32>) {
        let ( mut set_one, mut set_two ): (Vec<i32>, Vec<i32>) = self.file_contents
            .lines()
            .map(|l| {
                let parts = l.split("   ").collect::<Vec<&str>>();
                let first = parts[0];
                let second = parts[1];

                (first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap())
            })
            .unzip();

        // sort both
        set_one.sort();
        set_two.sort();

        (set_one, set_two)
    }
}

fn parser(line: &str) -> bool {
    let re = Regex::new(r"mul\(?P<position_one>[0-9]{1,3},?P<position_one>[0-9]{1,3}\)/g").unwrap();
    let matches = re.captures(line).unwrap();

    let position_one = matches.name("position_one").unwrap();
    let position_two = matches.name("position_two").unwrap();


    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let res =  parser(input);

        assert_eq!(res, true)
    }

    // #[test]
    fn pt1_sample_input_test() {
        let day_three = DayThree::new("./inputs/day3/sample");
        let answer = day_three.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(11, i);
            return;
        }

        panic!()
    }


    // #[test]
    fn pt1_full_sample_input_test() {
        let day_one = DayThree::new("./inputs/day1/full");
        let answer = day_one.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(1506483, i, "Asserting integer response of 1506483 to match test input");
            return;
        }

        panic!("Expected integer type, with value of 1506483")
    }

    // #[test]
    fn pt2_sample_input_test() {
        let day_one = DayThree::new("./inputs/day1/sample");
        let answer = day_one.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(31, i, "Asserting integer response of 31 to match test input");
            return;
        }

        panic!("Expected integer type")
    }

    // #[test]
    fn pt2_full_sample_input_test() {
        let day_one = DayThree::new("./inputs/day1/full");
        let answer = day_one.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(23126924, i);
            return;
        }

        panic!("Expected integer type")
    }
}
