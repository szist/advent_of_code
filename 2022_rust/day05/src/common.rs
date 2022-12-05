use regex::Regex;

pub fn parse_initial_configuration(initial: &str) -> Vec<Vec<u8>> {
    // we should flip the "matrix"
    let lines = initial
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    // ignore the last line of stack numbers
    let initial_stack_size = lines.len() - 1;
    let stack_count = (lines.last().unwrap().len() + 2) / 4; // "[1] "
    let mut stacks: Vec<Vec<u8>> = Vec::with_capacity(stack_count);
    // initialize the stacks
    for _ in 0..stack_count {
        stacks.push(Vec::with_capacity(initial_stack_size))
    }
    // fill in stacks
    for depth in (0..initial_stack_size).rev() {
        for stack_index in 0..stack_count {
            let v = lines[depth][stack_index * 4 + 1];
            if v == ' ' as u8 {
                continue;
            }
            stacks[stack_index].push(v);
        }
    }
    stacks
}

fn unwrap_match_to_usize(re_match: Option<regex::Match<'_>>) -> usize {
    re_match
        .unwrap()
        .as_str()
        .parse()
        .expect("Expected a number here")
}

#[derive(PartialEq, Debug)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_moves(moves: &str) -> Vec<Move> {
    let move_reg = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Bad regexp");
    moves
        .trim()
        .split('\n')
        .map(|line| {
            let captures = move_reg.captures(line).expect("Line does not match a move");
            Move {
                count: unwrap_match_to_usize(captures.get(1)),
                from: unwrap_match_to_usize(captures.get(2)) - 1,
                to: unwrap_match_to_usize(captures.get(3)) - 1,
            }
        })
        .collect()
}

pub fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Move>) {
    let separator = input
        .find("\n\n")
        .expect("Unable to find separator between initial configuration and moves");
    let (initial, moves) = input.trim_end().split_at(separator);
    let stacks = parse_initial_configuration(initial);
    let moves = parse_moves(moves);
    (stacks, moves)
}

pub fn top_crates(stacks: &Vec<Vec<u8>>) -> String {
    let mut headers = String::new();
    for stack in stacks {
        headers.push(*stack.last().expect("Empty final stack") as char);
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial() {
        let input = "    [D]    \n\
[N] [C]    \n\
[Z] [M] [P]\n\
 1   2   3 ";
        assert_eq!(
            parse_initial_configuration(input),
            vec![
                vec!['Z' as u8, 'N' as u8],
                vec!['M' as u8, 'C' as u8, 'D' as u8],
                vec!['P' as u8]
            ]
        );
    }

    #[test]
    fn check_moves() {
        let moves = "\n\
move 1 from 2 to 1\n\
move 3 from 1 to 3";
        assert_eq!(
            parse_moves(moves),
            vec![
                Move {
                    count: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    count: 3,
                    from: 1,
                    to: 3
                }
            ]
        )
    }
}
