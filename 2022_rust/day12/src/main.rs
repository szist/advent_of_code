use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input!");
    let matrix = common::parse_input_matrix(input.trim_end());
    println!("Task 1: {}", task1::solve(&matrix));
    println!("Task 2: {}", task2::solve(&matrix));
}
