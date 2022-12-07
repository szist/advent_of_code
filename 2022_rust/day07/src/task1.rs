use std::collections::HashMap;

use crate::common::{dfs, parse_log};

pub fn solve(input: &str) -> usize {
    let root_dir = parse_log(input);

    let mut result: HashMap<String, usize> = HashMap::new();
    dfs(&root_dir, "", &mut result);

    return result.values().filter(|v| **v < 100000).sum();
}
