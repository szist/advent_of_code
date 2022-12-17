use std::cmp::Ordering;

use crate::common;

pub fn height(chamber: &Vec<Vec<usize>>) -> usize {
    chamber.iter().filter(|r| r.iter().any(|&b| b > 0)).count() - 1
}

const REQUIRED_ROCKS: usize = 1_000_000_000_000;

pub fn solve(jets: &[u8], jet_count: usize) -> Option<usize> {
    let mut jets = jets.iter().cycle();
    let mut chamber = vec![vec![2; 7]];
    let potential_iter = jet_count;

    let mut diff_patt: Vec<Vec<usize>> = vec![];
    let mut last_patt_rock = 0;
    let mut rock_diff = 0;
    let mut rocks_fallen = 0;

    // All this sh*t below is because I probably made some mistake while actually
    // trying to detect a stable cycle with moves and shapes...
    // So here is an empirical one
    for rock in 0..potential_iter * 100 {
        common::drop(&mut chamber, &common::SHAPES[rock % 5], &mut jets);

        // try to find matching rows
        if chamber.len() > potential_iter * 2 && diff_patt.len() == 0 {
            let offset = 100;
            let mut found_index = 0;
            'search: for begin in 100 + offset..chamber.len() {
                for patt in 0..potential_iter {
                    if chamber[patt + offset].cmp(&chamber[begin + patt]) != Ordering::Equal {
                        continue 'search;
                    }
                }
                if found_index > 0 {
                    println!("Found {} index {}", found_index, begin);
                    chamber[found_index..begin]
                        .iter()
                        .for_each(|x| diff_patt.push(x.clone()));
                    break;
                } else {
                    found_index = begin;
                }
            }
        }

        if diff_patt.len() > 0 {
            let offset = 100;
            let mut found_pat = true;
            for patt in 0..diff_patt.len() {
                if chamber[patt + offset].cmp(&diff_patt[patt]) != Ordering::Equal {
                    found_pat = false;
                    break;
                }
            }
            if found_pat {
                if last_patt_rock > 0 {
                    rock_diff = rock - last_patt_rock;
                    rocks_fallen = rock;
                    break;
                } else {
                    last_patt_rock = rock.clone();
                }
            }
        }
    }

    println!("Found pattern {} rock diff: {}", diff_patt.len(), rock_diff);

    let missing_rocks = REQUIRED_ROCKS - rocks_fallen;

    let extra_height = (missing_rocks / rock_diff) * diff_patt.len();

    let extra_rocks = missing_rocks % rock_diff;

    for rock in rocks_fallen + 1..rocks_fallen + extra_rocks {
        common::drop(&mut chamber, &common::SHAPES[rock % 5], &mut jets);
    }

    Some(height(&chamber) + extra_height)
}
