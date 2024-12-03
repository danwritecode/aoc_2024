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
            let first = l[0];
            let last = l[l.len() - 1];
            let is_increasing = first < last;

            for (i, v) in l.iter().enumerate() {
                // reached the end can no longer evaluate
                // we know it's safe now
                if i == l.len() - 1 {
                    ct_safe += 1;
                    break;
                }

                // calulate disqualifying conditions
                let next = l[i + 1];
                let valid = evaluate_next(v, &next, is_increasing);

                if !valid {
                    break;
                }
            }
        }


        return AdventReturn::Integer(ct_safe);
    }

    fn process_pt2(&self) -> AdventReturn {
        let lines = &self.parse();
        let mut ct_safe = 0;

        for l in lines {
            let first = l[0];
            let last = l[l.len() - 1];
            let is_increasing = first < last;
            let mut ct_unsafe = 0;

            for (i, v) in l.iter().enumerate() {
                if i == l.len() - 1 {
                    break;
                }

                let next = l[i + 1];

                // if next is the end then we can safely drop it
                if i + 1 as usize == l.len() - 1 {
                    break;
                }

                let next_valid = evaluate_next(v, &next, is_increasing);
                let skipped_next = l[i + 2];
                let skipped_next_valid = evaluate_next(v, &skipped_next, is_increasing);
                
                if !next_valid {
                    ct_unsafe += 1;
                }

                if !skipped_next_valid {
                    ct_unsafe += 1;
                }
            }

            let line_str = l.iter().map(|c| c.to_string()).collect::<String>();
            println!("unsafe: {} | for line: {:?}", ct_unsafe, line_str);

            if ct_unsafe <= 1 {
                ct_safe += 1;
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
}

fn evaluate_next(current: &i32, next: &i32, is_increasing: bool) -> bool {
    let diff = current.abs_diff(*next);

    if diff < 1 || diff > 3 {
        return false;
    }

    if is_increasing {
        if current > next {
            return false;
        }
        return true
    } else {
        if current < next {
            return false;
        }
        return true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_pt_1_sample_input_test() {
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
    fn day2_pt_2_sample_input_test() {
        let day_two = DayTwo::new("./inputs/day2/sample");
        let answer = day_two.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(4, i,);
            return;
        }

        panic!("Expected integer type")
    }

    // #[test]
    // fn day2_pt2_full_sample_input_test() {
    //     let day_two = DayTwo::new("./inputs/day2/full");
    //     let answer = day_two.process_pt2();
    //
    //     if let AdventReturn::Integer(i) = answer {
    //         assert_eq!(287, i,);
    //         return;
    //     }
    //
    //     panic!("Expected integer type")
    // }
}
