use std::collections::HashSet;

use crate::common::{manhattan, parse_sensors, Pos};

pub fn solve(input: &str, row: isize) -> usize {
    let sensors = parse_sensors(input);
    let min_with_buffer = sensors
        .iter()
        .map(|sensor| sensor.pos.0 - sensor.range)
        .min()
        .unwrap()
        - 1;
    let max_with_buffer = sensors
        .iter()
        .map(|sensor| sensor.pos.0 + sensor.range)
        .max()
        .unwrap()
        + 1;

    let beacon_sensors: HashSet<isize> = sensors
        .iter()
        .map(|s| {
            if s.beacon.1 == row {
                Some(s.beacon.0)
            } else if s.pos.1 == row {
                Some(s.pos.0)
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    let mut unreachable_count = 0;

    // this is a bruteforce solution. The better approach would be to
    // have an interval per sensor and then just merge the intervals
    for x in min_with_buffer..max_with_buffer {
        let cpos = Pos(x, row);
        for sensor in &sensors {
            let distance = manhattan(&sensor.pos, &cpos);
            if distance - sensor.range <= 0 {
                unreachable_count += 1;
                break;
            }
        }
    }
    unreachable_count - beacon_sensors.len()
}
