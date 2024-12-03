use std::fmt::Display;

pub mod day1;
pub mod day2;

#[derive(Debug)]
enum AdventReturn {
    String(String),
    Integer(i32),
    Float(f64)
}

impl Display for AdventReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdventReturn::String(s) => write!(f, "{}", s),
            AdventReturn::Integer(i) => write!(f, "{}", i),
            AdventReturn::Float(fl) => write!(f, "{}", fl),
        }
    }
}

trait AdventDay {
    fn new(input_path: &str) -> Self;
    fn process_pt1(&self) -> AdventReturn;
    fn process_pt2(&self) -> AdventReturn;
}

fn main() {
    println!("Hello, advent of code!");
}
