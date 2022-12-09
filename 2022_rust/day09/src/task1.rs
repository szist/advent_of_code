use std::collections::HashSet;

use crate::common;

pub fn solve(moves: &Vec<common::Move>) -> usize {
    let mut visited: HashSet<common::Point> = HashSet::new();

    let mut pos_h = common::Point { x: 0, y: 0 };
    let mut pos_t = common::Point { x: 0, y: 0 };
    for m in moves {
        for _ in 0..m.steps {
            common::execute_step(&m.dir, &mut pos_h);
            common::pull(&mut pos_h, &mut pos_t);
            visited.insert(pos_t.clone());
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    #[test]
    fn check_solve() {
        assert_eq!(
            solve(&vec![
                Move {
                    dir: Direction::RIGHT,
                    steps: 4
                },
                Move {
                    dir: Direction::UP,
                    steps: 4
                },
                Move {
                    dir: Direction::LEFT,
                    steps: 3
                },
                Move {
                    dir: Direction::DOWN,
                    steps: 1
                },
                Move {
                    dir: Direction::RIGHT,
                    steps: 4
                },
                Move {
                    dir: Direction::DOWN,
                    steps: 1
                },
                Move {
                    dir: Direction::LEFT,
                    steps: 5
                },
                Move {
                    dir: Direction::RIGHT,
                    steps: 2
                }
            ]),
            13
        );
    }
}
