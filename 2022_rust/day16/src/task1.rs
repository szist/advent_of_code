use std::collections::HashMap;

use crate::common::{max_flow, Valve};

pub fn solve(valves: &HashMap<String, Valve>) -> Option<usize> {
    max_flow(valves, 30)
}
