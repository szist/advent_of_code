use std::cmp;

#[derive(Debug, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Move {
    pub dir: Direction,
    pub steps: usize,
}

pub fn parse_moves(input: &str) -> Vec<Move> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (dir, steps) = line.split_once(' ').unwrap();
            let dir = match dir {
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                _ => unreachable!("Bad move"),
            };
            let steps: usize = steps.parse().unwrap();
            Move { dir, steps }
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub fn execute_step(dir: &Direction, h: &mut Point) {
    match dir {
        Direction::DOWN => h.y -= 1,
        Direction::UP => h.y += 1,
        Direction::RIGHT => h.x += 1,
        Direction::LEFT => h.x -= 1,
    }
}

// pulls closer point b to point a if too far.
pub fn pull(a: &Point, b: &mut Point) {
    let dist_x = b.x - a.x;
    let dist_y = b.y - a.y;
    if cmp::max(dist_x.abs(), dist_y.abs()) >= 2 {
        b.x = a.x + if dist_x.abs() > 1 { dist_x.signum() } else { 0 };
        b.y = a.y + if dist_y.abs() > 1 { dist_y.signum() } else { 0 };
    }
}
