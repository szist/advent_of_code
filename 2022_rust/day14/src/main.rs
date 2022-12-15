use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (grid, offset) = common::parse_input_lines(&input);
    println!("Task 1 : {}", task1::solve(&grid, offset));
    println!("Task 2 : {}", task2::solve(&grid, offset));
}
