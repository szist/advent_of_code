use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input!");
    let instructions = common::parse_instructions(&input);
    println!("Task 1 signal strength: {}", task1::solve(&instructions));
    task2::solve(&instructions);
}
