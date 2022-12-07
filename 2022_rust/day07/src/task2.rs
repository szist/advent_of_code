use std::collections::HashMap;

use crate::common::{dfs, parse_log};

pub fn solve(input: &str) -> usize {
    let root_dir = parse_log(input);

    let mut result: HashMap<String, usize> = HashMap::new();
    let used_size = dfs(&root_dir, "", &mut result);

    let mut sorted = result.iter().collect::<Vec<_>>();

    sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    let free_space = 70000000 - used_size;
    let required_space = 30000000 - free_space;

    println!(
        "Free space: {} Required to clean up: {}",
        free_space, required_space
    );

    for (key, size) in sorted {
        if *size > required_space {
            println!("Found folder {}", key);
            return *size;
        }
    }

    0
}
