use crate::common::{parse_input, top_crates, Move};

pub fn solve(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for Move { count, from, to } in moves {
        for _ in 0..count {
            let val = stacks[from].pop().expect("Stack was already empty");
            stacks[to].push(val);
        }
    }
    top_crates(&stacks)
}
