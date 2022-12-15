use std::collections::HashSet;

use crate::common::{manhattan, parse_sensors, Pos};

pub fn solve(input: &str) -> isize {
    let sensors = parse_sensors(input);

    let mut pos = Pos(0, 0);
    const MIN: isize = 0;
    const MAX: isize = 4_000_000;

    // This is kind of slow 4m * N^2
    // In an ideal case it should be much more efficient finding
    // the intersactions between 2 and checking against a 3rd sensor range
    'all: for row in 0..4_000_000 {
        let mut points: HashSet<isize> = HashSet::new();
        for sensor in &sensors {
            let dist_from_row = (row - sensor.pos.1).abs();
            if sensor.range >= dist_from_row {
                let remaining = sensor.range - dist_from_row;
                let start = sensor.pos.0 - remaining - 1;
                let end = sensor.pos.0 + remaining + 1;
                if end >= MIN && end <= MAX {
                    points.insert(end);
                }
                if start >= MIN && start <= MAX {
                    points.insert(start);
                }
            }
        }
        'ends: for point in points {
            let current = Pos(point, row);
            for sensor in &sensors {
                if manhattan(&sensor.pos, &current) <= sensor.range {
                    continue 'ends;
                }
            }
            pos = current;
            break 'all;
        }
    }

    pos.0 * 4000000 + pos.1
}
