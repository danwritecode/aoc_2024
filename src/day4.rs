use std::fs::read_to_string;
use crate::AdventReturn;

pub struct DayFour {
    file_contents: String,
    matrix: Vec<Vec<char>>
}

impl DayFour {
    fn new(input_path: &str) -> Self {
        let file_contents = read_to_string(input_path).expect("Could not read file contents");
        let matrix = DayFour::parse(&file_contents);
        DayFour { file_contents, matrix } 
    }

    fn process_pt1(&self) -> AdventReturn {
        let mut ct = 0;
        for (xi, row) in self.matrix.iter().enumerate() {
            for (yi, _) in row.iter().enumerate() {
                ct += self.eval_xmas_ct(xi, yi);
            }
        }

        AdventReturn::Integer(ct as i32)
    }

    fn process_pt2(&self) -> AdventReturn {
        let mut ct = 0;
        for (xi, row) in self.matrix.iter().enumerate() {
            for (yi, _) in row.iter().enumerate() {
                if self.eval_x_mas_ct(xi, yi) == true { ct += 1 };
            }
        }

        AdventReturn::Integer(ct as i32)
    }
}

enum EvalDirection {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl DayFour {
    fn parse(file_contents: &String) -> Vec<Vec<char>> {
        file_contents
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    fn eval_xmas_ct(&self, x: usize, y: usize) -> usize {
        let mut ct = 0;
        if self.eval_word_direction(x, y, EvalDirection::Up, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::Down, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::Right, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::Left, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::UpRight, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::UpLeft, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::DownRight, "XMAS") == true { ct += 1};
        if self.eval_word_direction(x, y, EvalDirection::DownLeft, "XMAS") == true { ct += 1};

        ct
    }

    fn eval_x_mas_ct(&self, x: usize, y: usize) -> bool {
        let mut ct = 0;
        let char = match self.get_coord_char(x, y) {
            Some(c) => c,
            None => return false
        };

        if char != 'A' {
            return false;
        }

        if x > 0 && y > 0 {
            let ul_cord = (x - 1, y - 1);
            if self.eval_word_direction(ul_cord.0, ul_cord.1, EvalDirection::DownRight, "MAS") == true { ct += 1};
        }

        if x > 0 {
            let dl_cord = (x - 1, y + 1);
            if self.eval_word_direction(dl_cord.0, dl_cord.1, EvalDirection::UpRight, "MAS") == true { ct += 1};
        }

        if y > 0 {
            let ur_cord = (x + 1, y - 1);
            if self.eval_word_direction(ur_cord.0, ur_cord.1, EvalDirection::DownLeft, "MAS") == true { ct += 1};
        }

        let br_cord = (x + 1, y + 1);
        if self.eval_word_direction(br_cord.0, br_cord.1, EvalDirection::UpLeft, "MAS") == true { ct += 1};

        ct >= 2
    }


    fn eval_word_direction(&self, x: usize, y: usize, direction: EvalDirection, match_word: &str) -> bool {
        let char = match self.get_coord_char(x, y) {
            Some(c) => c,
            None => return false
        };

        let first_char = match_word.chars().nth(0).unwrap();

        if char != first_char {
            return false
        }

        let mut chars = vec![first_char];

        for c in 0..match_word.len() - 1 {
            match direction {
                EvalDirection::Up => {
                    match self.get_above(x, y - c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::Down => {
                    match self.get_below(x, y + c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::Left => {
                    match self.get_left(x - c, y) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::Right => {
                    match self.get_right(x + c, y) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::UpRight => {
                    match self.get_diag_up_right(x + c, y - c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::UpLeft => {
                    match self.get_diag_up_left(x - c, y - c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::DownRight => {
                    match self.get_diag_down_right(x + c, y + c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
                EvalDirection::DownLeft => {
                    match self.get_diag_down_left(x - c, y + c) {
                        Some(c) => chars.push(c),
                        None => return false
                    }
                },
            }
        }

        let chars = chars.iter().collect::<String>();
        return &chars == match_word;
    }

    fn get_above(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Up) {
            return None;
        }

        let target = self.matrix[y - 1][x];

        Some(target)
    }

    fn get_below(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Down) {
            return None;
        }

        let target = self.matrix[y + 1][x];
        Some(target)
    }

    fn get_right(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Right) {
            return None;
        }

        let target = self.matrix[y][x + 1];
        Some(target)
    }

    fn get_left(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Left) {
            return None;
        }

        let target = self.matrix[y][x - 1];
        Some(target)
    }

    fn get_diag_up_right(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::UpRight) {
            return None;
        }

        if let false = self.can_move(x, y, EvalDirection::Right) {
            return None;
        }

        let target = self.matrix[y - 1][x + 1];

        Some(target)
    }

    fn get_diag_up_left(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Up) {
            return None;
        }

        if let false = self.can_move(x, y, EvalDirection::Left) {
            return None;
        }

        let target = self.matrix[y - 1][x - 1];

        Some(target)
    }

    fn get_diag_down_right(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Down) {
            return None;
        }

        if let false = self.can_move(x, y, EvalDirection::Right) {
            return None;
        }

        let target = self.matrix[y + 1][x + 1];

        Some(target)
    }

    fn get_diag_down_left(&self, x: usize, y: usize) -> Option<char> {
        if let false = self.can_move(x, y, EvalDirection::Down) {
            return None;
        }

        if let false = self.can_move(x, y, EvalDirection::Left) {
            return None;
        }

        let target = self.matrix[y + 1][x - 1];

        Some(target)
    }

    fn can_move(&self, x: usize, y: usize, direction: EvalDirection) -> bool {
        match direction {
            EvalDirection::Up => y != 0,
            EvalDirection::Down => y < self.matrix.len() - 1,
            EvalDirection::Left => x != 0,
            EvalDirection::Right => x < self.matrix[y].len() - 1,
            EvalDirection::UpRight => y != 0 && x < self.matrix[y].len() - 1,
            EvalDirection::UpLeft => y != 0 && x != 0,
            EvalDirection::DownRight => y < self.matrix.len() - 1 && x < self.matrix[y].len() - 1,
            EvalDirection::DownLeft => y < self.matrix.len() - 1 && x != 0,
        }
    }

    fn get_coord_char(&self, x: usize, y: usize) -> Option<char> {
        if y > self.matrix.len() - 1 {
            return None;
        }

        if x > self.matrix[y].len() - 1 {
            return None;
        }

        Some(self.matrix[y][x])
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_day_4_pt_1_sample() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let answer = day_four.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(18, i);
            return;
        }

        panic!()
    }

    #[test]
    fn eval_day_4_pt_1_full() {
        let day_four = DayFour::new("./inputs/day4/full");
        let answer = day_four.process_pt1();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(2567, i);
            return;
        }

        panic!()
    }

    #[test]
    fn eval_day_4_pt_2_sample() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let answer = day_four.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(9, i);
            return;
        }

        panic!()
    }

    #[test]
    fn eval_day_4_pt_2_full() {
        let day_four = DayFour::new("./inputs/day4/full");
        let answer = day_four.process_pt2();

        if let AdventReturn::Integer(i) = answer {
            assert_eq!(2029, i);
            return;
        }

        panic!()
    }

    #[test]
    fn eval_up_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(9, 9, EvalDirection::Up, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_up_right_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(5, 9, EvalDirection::UpRight, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_up_left_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(9, 9, EvalDirection::UpLeft, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_down_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(9, 3, EvalDirection::Down, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_down_right_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(4, 0, EvalDirection::DownRight, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_down_left_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(9, 3, EvalDirection::DownLeft, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_right_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(5, 0, EvalDirection::Right, "XMAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_left_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(4, 1, EvalDirection::Left, "XMAS");
        assert_eq!(true, result);
    }



    // x-mas
    #[test]
    fn eval_up_right_mas_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(1, 2, EvalDirection::UpRight, "MAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_up_left_mas_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(8, 3, EvalDirection::UpLeft, "MAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_down_right_mas_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(1, 0, EvalDirection::DownRight, "MAS");
        assert_eq!(true, result);
    }

    #[test]
    fn eval_down_left_mas_test_day_4() {
        let day_four = DayFour::new("./inputs/day4/sample");
        let result = day_four.eval_word_direction(7, 1, EvalDirection::DownLeft, "MAS");
        assert_eq!(true, result);
    }
}
