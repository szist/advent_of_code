use std::fs;

mod common;
mod task1;
mod task2;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let cubes: Vec<(usize, usize, usize)> = input
        .trim()
        .lines()
        .map(|l| -> (usize, usize, usize) {
            let elems: Vec<&str> = l.split(",").collect();
            (
                elems[0].parse().unwrap(),
                elems[1].parse().unwrap(),
                elems[2].parse().unwrap(),
            )
        })
        .collect();
    println!("Task 1 : {}", task1::solve(&cubes).unwrap());
    println!("Task 2 : {}", task2::solve(&cubes).unwrap());
}
