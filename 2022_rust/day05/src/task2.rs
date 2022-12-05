use crate::common::{parse_input, top_crates, Move};

pub fn solve(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for Move { count, from, to } in moves {
        let mut intermediate: Vec<u8> = Vec::with_capacity(count);
        for _ in 0..count {
            let val = stacks[from].pop().expect("Stack was already empty");
            intermediate.push(val);
        }
        for _ in 0..count {
            let val = intermediate.pop().unwrap();
            stacks[to].push(val);
        }
    }
    top_crates(&stacks)
}
