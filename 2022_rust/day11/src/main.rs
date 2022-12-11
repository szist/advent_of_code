use crate::input::get_monkeys;

mod input;
mod task;

fn main() {
    let common_multiple = get_monkeys()
        .iter()
        .map(|m| m.divisor)
        .reduce(|acc, item| acc * item)
        .unwrap();

    println!("Task 1: {}", task::solve(task::InspectMode::DIVIDE));
    println!(
        "Task 2: {}",
        task::solve(task::InspectMode::NODIVIDE(common_multiple))
    );
}
