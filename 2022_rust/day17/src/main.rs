use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let jets = fs::read_to_string("./input.txt").unwrap();
    let jetcount = jets.trim().len();
    let jets = jets.trim().as_bytes();
    println!("Task 1 : {}", task1::solve(jets).unwrap());
    println!("Task 2 : {}", task2::solve(jets, jetcount).unwrap());
}
