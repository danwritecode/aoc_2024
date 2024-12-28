use std::fs::read_to_string;
use crate::{AdventDay, AdventReturn};


pub struct DayTwo {
    file_contents: String
}

impl AdventDay for DayTwo {
    fn new(input_path: &str) -> Self {
        let file_contents = read_to_string(input_path).expect("Could not read file contents");
        DayTwo { file_contents } 
    }

    fn process_pt1(&self) -> AdventReturn {
        let lines = &self.parse();
        let mut ct_safe = 0;

        for l in lines {
            match Self::row_is_valid(l) {
                true => ct_safe += 1,
                false => ()
            }
        }

        return AdventReturn::Integer(ct_safe);
    }

    fn process_pt2(&self) -> AdventReturn {
        let lines = &self.parse();
        let mut ct_safe = 0;

        for l in lines {
            match Self::row_is_valid(l) {
                true => ct_safe += 1,
                false => {
                    for (i, _) in l.iter().enumerate() {
                        let mut lines_cpy = l.clone();
                        lines_cpy.remove(i); 

                        if let true = Self::row_is_valid(&lines_cpy) {
                            ct_safe += 1;
                            break;
                        }
                    }
                }
            }
        }

        return AdventReturn::Integer(ct_safe);
    }

}

impl DayTwo {
    fn parse(&self) -> Vec<Vec<i32>> {
        let lines = self.file_contents
            .lines()
            .map(|l| l.split(" ").map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

        lines
    }

    fn row_is_valid(row: &Vec<i32>) -> bool {
        let first = row[0];
        let last = row[row.len() - 1];
        let is_increasing = first < last;

        for (i, v) in row.iter().enumerate() {
            if i == row.len() - 1 {
                break;
            }

            // calulate disqualifying conditions
            let next_val = row[i + 1];
            match Self::is_next_valid(v, &next_val, is_increasing) {
                true => (),
                false => return false
            }
        }

        return true;
    }

    fn is_next_valid(current: &i32, next: &i32, is_increasing: bool) -> bool {
        let diff = current.abs_diff(*next);
        const MIN_DIFF: u32 = 1;
        const MAX_DIFF: u32 = 3;

        if diff < MIN_DIFF || diff > MAX_DIFF {
            return false;
        }

        if is_increasing {
            if current > next {
                return false;
            }
            return true;
        } else {
            if current < next {
                return false;
            }
            return true;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_pt1_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/sample");
        let answer = day_two.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(2, i,);
            return;
        }

        panic!("Expected integer type")
    }

    #[test]
    fn day2_pt1_full_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/full");
        let answer = day_two.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(287, i,);
            return;
        }

        panic!("Expected integer type")
    }

    #[test]
    fn day2_pt2_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/sample");
        let answer = day_two.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(5, i);
            return;
        }

        panic!("Expected integer type")
    }

    #[test]
    fn day2_pt2_full_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/full");
        let answer = day_two.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(354, i,);
            return;
        }

        panic!("Expected integer type")
    }
}
