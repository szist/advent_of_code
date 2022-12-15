use crate::common::{drop_sand, Field, Matrix};

pub fn solve(grid: &Matrix, source: usize) -> usize {
    let mut stuck = 0;
    let mut grid = grid.clone();
    loop {
        // drop sand

        if let Some(pos) = drop_sand(&grid, source) {
            *grid.get_mut(pos.0, pos.1) = Field::Sand;
            stuck += 1;
            continue;
        }
        grid.print();
        break;
    }
    let count = grid.data.iter().filter(|&x| *x == Field::Sand).count();
    assert!(count == stuck, "Count and stuck sand is equal");

    stuck
}
