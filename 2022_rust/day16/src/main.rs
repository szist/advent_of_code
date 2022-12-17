use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let valves = common::parse_valves(&input);
    println!("Task 1 : {}", task1::solve(&valves).unwrap());
    println!("Task 2 : {}", task2::solve(&valves).unwrap());
}
