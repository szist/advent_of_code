use std::fs;

mod common;
mod task1;
mod task2;

/**
 * In this round playing around mostly with modules.
 * The way I'm handling them is probably an antipattern :/
 * bit annoying how the structure of modules needs to be explicit
 */

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result_task1 = task1::solve(&input);
    println!("Task 1: {}", result_task1);
    let result_task2 = task2::solve(&input);
    println!("Task 2: {}", result_task2);
}
