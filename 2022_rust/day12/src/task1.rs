use pathfinding::prelude::bfs;

use crate::common::{get_height, Matrix, END, START};

// Pos(col, row)
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, isize);

pub fn solve(matrix: &Matrix) -> usize {
    let width = matrix.width as isize;
    let height = matrix.height as isize;
    let start = matrix.data.iter().position(|x| *x == START).unwrap() as isize;
    let start = Pos(start % width, start / width);

    let res = bfs(
        &start,
        |p| {
            let current_height = get_height(matrix.get(p.0 as usize, p.1 as usize));
            vec![
                Pos(p.0 - 1, p.1),
                Pos(p.0, p.1 - 1),
                Pos(p.0 + 1, p.1),
                Pos(p.0, p.1 + 1),
            ]
            .into_iter()
            // filter bounds
            .filter(|p| p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height)
            // filter ability to step towards
            .filter(move |n| {
                let height = get_height(matrix.get(n.0 as usize, n.1 as usize));
                height - current_height <= 1
            })
        },
        |p| *matrix.get(p.0 as usize, p.1 as usize) == END,
    );

    res.unwrap().len() - 1 // steps, not edges
}
