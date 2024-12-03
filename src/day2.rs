use std::{char::MAX, f32::MIN, fs::read_to_string};
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
            match Self::evaluate(l) {
                Ok(_) => ct_safe += 1,
                Err(e) => ()
            }
        }

        return AdventReturn::Integer(ct_safe);
    }

    fn process_pt2(&self) -> AdventReturn {
        let lines = &self.parse();
        let mut ct_safe = 0;

        for l in lines {
            match Self::evaluate(l) {
                Ok(_) => ct_safe += 1,
                Err(idxs) => {
                    for i in idxs {
                        let mut lines_cpy = l.clone();
                        lines_cpy.remove(i); 

                        if let Ok(()) = Self::evaluate(&lines_cpy) {
                            ct_safe += 1;
                        }
                    }
                }
            }
        }

        // panic!();
        return AdventReturn::Integer(ct_safe);
    }

}

type FailureIndex = usize;

enum FailureIndexPos {
    Next,
    Current,
    CurrentOrNext,
    None
}

impl DayTwo {
    fn parse(&self) -> Vec<Vec<i32>> {
        let lines = self.file_contents
            .lines()
            .map(|l| l.split(" ").map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

        lines
    }

    fn evaluate(row: &Vec<i32>) -> Result<(), Vec<FailureIndex>> {
        let first = row[0];
        let last = row[row.len() - 1];
        let is_increasing = first < last;

        for (i, v) in row.iter().enumerate() {
            if i == row.len() - 1 {
                break;
            }

            // calulate disqualifying conditions
            let next_val = row[i + 1];
            match Self::evaluate_next(i, v, &next_val, is_increasing) {
                FailureIndexPos::Next => return Err(vec![i + 1]),
                FailureIndexPos::Current => return Err(vec![i]),
                FailureIndexPos::CurrentOrNext => return Err(vec![i, i + 1]),
                FailureIndexPos::None => (),
            }

            // match Self::evaluate_next(i, v, &next_val, is_increasing) {
            //     FailureIndexPos::Next => {
            //         println!("Failed on Next: {:?}", row);
            //         return Err(vec![i + 1]);
            //     },
            //     FailureIndexPos::Current => {
            //         println!("Failed on Current: {:?}", row);
            //         return Err(vec![i]);
            //     },
            //     FailureIndexPos::CurrentOrNext => {
            //         println!("Failed on CurrentAndNext: {:?}", row);
            //         return Err(vec![i, i + 1]);
            //     },
            //     FailureIndexPos::None => (),
            // }
        }

        return Ok(());
    }

    fn evaluate_next(index: usize, current: &i32, next: &i32, is_increasing: bool) -> FailureIndexPos {
        let diff = current.abs_diff(*next);
        const MIN_DIFF: u32 = 1;
        const MAX_DIFF: u32 = 3;

        if diff < MIN_DIFF || diff > MAX_DIFF {
            // if index == 0 {
            //     return FailureIndexPos::CurrentOrNext
            // }
            return FailureIndexPos::Current
        }

        if is_increasing {
            if current > next {
                return FailureIndexPos::Next
            }
            return FailureIndexPos::None
        } else {
            if current < next {
                return FailureIndexPos::Next
            }
            return FailureIndexPos::None
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

    // #[test]
    fn day2_pt2_full_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/full");
        let answer = day_two.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(334, i,);
            return;
        }

        panic!("Expected integer type")
    }
}
