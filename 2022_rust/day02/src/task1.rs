
use super::common::parse_line;

/**
 * points:
 * A - rock = 1
 * B - paper = 2
 * C - scissor = 3
 * 
 * first task
 * X - rock
 * Y - paper
 * Z - scissor
 * 
 * A > C, C > B, B > A
 */

const A: u32 = 'A' as u32;
const X: u32 = 'X' as u32;

/**
 * Playing around with mapping also with some math
 */
fn eval_round(opponent: u32, mine: u32) -> u32 {
    let opponent_normalized = opponent - A;
    let mine_normalized = mine - X;

    let diff = (mine_normalized + 3 - opponent_normalized) % 3;
    let won_score = match diff {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("Never happens")
    };
    let choice_score = mine_normalized + 1;
    return won_score + choice_score;
}


pub fn solve(input: &String) -> u32 {
    let mut score = 0;
    for line in input.split('\n') {
        match parse_line (line) {
            Ok((opponent, mine)) => {
                score += eval_round(opponent, mine);
            },
            _ => continue
        }
    }
    return score;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eval_round_test() {
        assert_eq!(eval_round('A' as u32, 'X' as u32), 4, "Rock equals");
        assert_eq!(eval_round('B' as u32, 'Y' as u32), 5, "Paper equals");
        assert_eq!(eval_round('C' as u32, 'Z' as u32), 6, "Scissor equals");

        assert_eq!(eval_round('C' as u32, 'X' as u32), 7, "Rock beats scissors");
        assert_eq!(eval_round('A' as u32, 'Z' as u32), 3, "Scissor looses to rock");
    }
}
