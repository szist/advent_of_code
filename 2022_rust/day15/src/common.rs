use regex::Regex;

fn unwrap_match_to_isize(re_match: Option<regex::Match<'_>>) -> isize {
    re_match
        .unwrap()
        .as_str()
        .parse()
        .expect("Expected a number here")
}

// Pos(col, row)
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub isize, pub isize);

pub struct Sensor {
    pub pos: Pos,
    pub beacon: Pos,
    pub range: isize,
}

pub fn parse_sensors(input: &str) -> Vec<Sensor> {
    let move_reg =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .expect("Bad regexp");
    input
        .trim_end()
        .lines()
        .map(|line| {
            let captures = move_reg.captures(line).expect("Line does not match a move");
            let sensor_x = unwrap_match_to_isize(captures.get(1));
            let sensor_y = unwrap_match_to_isize(captures.get(2));
            let beacon_x = unwrap_match_to_isize(captures.get(3));
            let beacon_y = unwrap_match_to_isize(captures.get(4));
            let pos = Pos(sensor_x, sensor_y);
            let beacon = Pos(beacon_x, beacon_y);
            Sensor {
                range: manhattan(&pos, &beacon),
                pos,
                beacon,
            }
        })
        .collect()
}

pub fn manhattan(a: &Pos, b: &Pos) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
