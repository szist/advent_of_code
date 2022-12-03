use std::collections::HashSet;
use crate::common::{compute_priority, parse_rucksack};

pub fn solve(input: &String) -> u32 {
    let mut priority_sum = 0;
    let rucksacks: Vec<&str> = input.split('\n').collect();
    for chunk in rucksacks.chunks(3) {
        if chunk.len() != 3 {
            // happens probably at the end of the file
            continue;
        }
        let elf1_items = parse_rucksack(chunk[0]);
        let elf2_items = parse_rucksack(chunk[1]);
        let elf3_items = parse_rucksack(chunk[2]);

        let first_two: HashSet<char> = elf1_items.intersection(&elf2_items).map(|x| *x).collect();
        let all_three: HashSet<char> = first_two.intersection(&elf3_items).map(|x| *x).collect();

        for item in all_three {
            priority_sum += compute_priority(item);
        }
    }
    priority_sum
}