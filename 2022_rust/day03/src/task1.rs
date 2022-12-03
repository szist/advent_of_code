
use super::common::{parse_compartments, compute_priority};

pub fn solve(input: &String) -> u32 {
    let mut priority_sum = 0;
    for line in input.split('\n') {
        let (comp_a, comp_b) = parse_compartments(line);
        let common_items = comp_a.intersection(&comp_b);
        for item in common_items {
            priority_sum += compute_priority(*item);
        }
    }
    priority_sum
}