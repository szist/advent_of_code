use std::collections::HashSet;

use crate::common;

pub fn solve(moves: &Vec<common::Move>) -> usize {
    let mut visited: HashSet<common::Point> = HashSet::new();

    let mut knots = vec![common::Point { x: 0, y: 0 }; 10];
    for m in moves {
        for _ in 0..m.steps {
            common::execute_step(&m.dir, knots.get_mut(0).unwrap());
            for i in 1..knots.len() {
                common::pull(
                    &knots.get(i - 1).unwrap().clone(),
                    knots.get_mut(i).unwrap(),
                );
            }
            visited.insert(knots.last().unwrap().clone());
        }
    }

    visited.len()
}
