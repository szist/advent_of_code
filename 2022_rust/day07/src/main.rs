extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fs;

mod common;
mod parser;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input!");

    println!("Task 1: {}", task1::solve(&input));
    println!("Task 2: {}", task2::solve(&input));
}
