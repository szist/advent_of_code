use crate::common;

pub fn solve(jets: &[u8]) -> Option<usize> {
    let mut jets = jets.iter().cycle();
    let mut chamber = vec![vec![2; 7]];

    for rock in 0..2022 {
        common::drop(&mut chamber, &common::SHAPES[rock % 5], &mut jets);
    }

    Some(chamber.iter().filter(|r| r.iter().any(|&b| b > 0)).count() - 1)
}
