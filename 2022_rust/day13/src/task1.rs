use std::cmp::Ordering;

use crate::common::compare_packets;

pub fn solve(input: &str) -> usize {
    let mut sum_of_indices = 0;
    for (index, pair) in input.split("\n\n").enumerate() {
        if let Some((p1, p2)) = pair.split_once('\n') {
            if compare_packets(p1, p2) == Ordering::Less {
                sum_of_indices += index + 1;
            }
        }
    }

    sum_of_indices
}
