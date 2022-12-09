use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input!");
    let moves = common::parse_moves(&input);
    println!("Task 1: {}", task1::solve(&moves));
    println!("Task 2: {}", task2::solve(&moves));
}
