use crate::common::{register_states, Instruction};

pub fn solve(instructions: &[Instruction]) -> isize {
    let states = register_states(instructions);
    let mut acc: isize = 0;
    for cycle in (20..=220).step_by(40) {
        acc += (cycle as isize) * states[cycle - 1]; // indexing from 0
    }
    acc
}
