use std::{collections::HashMap, fs::read_to_string};
use crate::{AdventDay, AdventReturn};


pub struct Foo {
    one: i32,
    two: i32
}

impl Foo {
    fn update_one(&mut self) {
        self.one += 1;

        // set two based on one
        self.two *= &self.one;
    }
}


pub struct DayOne {
    file_contents: String
}

impl AdventDay for DayOne {
    fn new(input_path: &str) -> Self {
        let file_contents = read_to_string(input_path).expect("Could not read file contents");
        DayOne { file_contents } 
    }

    fn process_pt1(&self) -> AdventReturn {
        let (set_one, set_two) = &self.parse();
        let diffs = set_one
            .iter()
            .zip(set_two.iter())
            .map(|(a, b)| a.abs_diff(*b) as i32)
            .collect::<Vec<i32>>();

        let diff = diffs.iter().sum::<i32>();

        return AdventReturn::Integer(diff);
    }

    fn process_pt2(&self) -> AdventReturn {
        let (set_one, set_two) = &self.parse();
        let mut key_sum = HashMap::new();

        for i in set_two {
            key_sum.entry(i).and_modify(|ct| *ct += 1).or_insert(1);
        }

        let score = set_one
            .iter()
            .map(|a| {
                let ct = key_sum.get(a).map_or(0, |b| *b);
                a * ct
            })
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>();

        AdventReturn::Integer(score)
    }
}

impl DayOne {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_1_sample_input_test() {
        let day_one = DayOne::new("./inputs/day1/sample");
        let answer = day_one.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(11, i, "Asserting integer response of 11 to match test input");
            return;
        }

        panic!("Expected integer type, with value of 11")
    }

    #[test]
    fn pt1_full_sample_input_test() {
        let day_one = DayOne::new("./inputs/day1/full");
        let answer = day_one.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(1506483, i, "Asserting integer response of 1506483 to match test input");
            return;
        }

        panic!("Expected integer type, with value of 1506483")
    }

    #[test]
    fn pt_2_sample_input_test() {
        let day_one = DayOne::new("./inputs/day1/sample");
        let answer = day_one.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(31, i, "Asserting integer response of 31 to match test input");
            return;
        }

        panic!("Expected integer type")
    }

    #[test]
    fn pt2_full_sample_input_test() {
        let day_one = DayOne::new("./inputs/day1/full");
        let answer = day_one.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(23126924, i);
            return;
        }

        panic!("Expected integer type")
    }
}
