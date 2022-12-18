use crate::common::surface;

pub fn solve(cubes: &Vec<(usize, usize, usize)>) -> Option<usize> {
    surface(cubes, true)
}
