
use super::common::parse_line;

const ROCK: u32 = 'A' as u32;
const PAPER: u32 = 'B' as u32;
const SCISSORS: u32 = 'C' as u32;
const LOSE: u32 = 'X' as u32;
const DRAW: u32 = 'Y' as u32;
const WIN: u32 = 'Z' as u32;

/* All of these helpers are not too efficient, just praciticing pattern matching */

fn eval_choice(choice: u32) -> u32 {
    return match choice {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => panic!("Bad choice")
    };
}

fn choose(opponent: u32, action: u32) -> u32 {
    let mine = match (opponent, action) {
        (_, DRAW) => opponent,
        (ROCK, LOSE) | (PAPER, WIN)  => SCISSORS,
        (PAPER, LOSE) | (SCISSORS, WIN)  => ROCK,
        (SCISSORS, LOSE) | (ROCK, WIN)  => PAPER,
        _ => panic!("Unexpected combination")
    };
    mine
}


fn eval_action(action: u32) -> u32 {
    let win_points = match action {
        LOSE => 0,
        DRAW => 3,
        WIN => 6,
        _ => panic!("bad inputs")
    };
    win_points
}


/**
 * This time around lets try only with pattern matching
 * 'X' means lost
 * 'Y' means draw
 * 'Z' means win
 */
fn eval_round(opponent: u32, action: u32) -> u32 {
    let win_points = eval_action(action);

    let mine = choose(opponent, action);

    let choice_points = eval_choice(mine);

    win_points + choice_points
}

pub fn solve(input: &String) -> u32 {
    let mut score = 0;
    for line in input.split('\n') {
        match parse_line (line) {
            Ok((opponent, action)) => {
                score += eval_round(opponent, action);
            },
            _ => continue
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_round_tests() {
        assert_eq!(eval_round(ROCK, LOSE), 3, "If need to lose and rock => paper with 3 points");
        assert_eq!(eval_round(PAPER, DRAW), 5, "Paper (2) + Draw (3) = 5 ");
    }

    #[test]
    fn check_choice() {
        assert_eq!(choose(ROCK, WIN), PAPER);
    }
}