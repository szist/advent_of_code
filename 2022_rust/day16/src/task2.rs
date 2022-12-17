use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::{max_flow, Valve};

const MAX_STEPS: usize = 26;

pub fn solve(valves: &HashMap<String, Valve>) -> Option<usize> {
    let valves_arr: Vec<Valve> = valves
        .values()
        .filter(|x| x.flow > 0)
        .map(Valve::clone)
        .collect();
    let mut max = 0;
    let start = "AA".to_string();

    // Instead fo trying to solve as a whole, split the valves into 2 groups
    // and solve separately. This way the trees will be relatively small to
    for split in 1..=valves_arr.len() / 2 {
        println!("Trying split at {split} - {}", valves_arr.len() - split);
        let combinations = (0..valves_arr.len()).combinations(split);
        for combination in combinations {
            let combination: HashSet<usize> = HashSet::from_iter(combination.iter().map(|x| *x));
            let mut left: HashMap<String, Valve> = HashMap::new();
            let mut right: HashMap<String, Valve> = HashMap::new();

            left.insert(start.clone(), valves.get(&start)?.clone());
            right.insert(start.clone(), valves.get(&start)?.clone());

            for index in 0..valves_arr.len() {
                let key = valves_arr[index].name.clone();
                let val = valves_arr[index].clone();
                (if combination.contains(&index) {
                    &mut left
                } else {
                    &mut right
                })
                .insert(key, val);
            }
            let left = max_flow(&left, MAX_STEPS)?;
            let right = max_flow(&right, MAX_STEPS)?;

            if left + right > max {
                max = left + right;
            }
        }
    }
    Some(max)
}
