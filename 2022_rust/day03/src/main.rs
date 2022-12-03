use std::fs;

mod common;
mod task1;
mod task2;

/**
 * Trying to do stuff through collections and automatic collect
 * The way I use `collect` and the lot though is probably very bad
 * and I will need to look into the referencing and the complexity of
 * these collections.
 */
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    
    println!("Task 1: {}", task1::solve(&input));
    println!("Task 2: {}", task2::solve(&input));
}
