use crate::common::{drop_sand, Field, Matrix};

pub fn solve(grid: &Matrix, source: usize) -> usize {
    let mut stuck = 0;
    let mut grid = grid.clone();
    // set the bottom
    for i in 0..grid.width {
        *grid.get_mut(i, grid.height - 1) = Field::Stone;
    }

    loop {
        // drop sand

        if let Some(pos) = drop_sand(&grid, source) {
            *grid.get_mut(pos.0, pos.1) = Field::Sand;
            stuck += 1;
            if pos.1 == 0 {
                grid.print();
                break;
            } else {
                continue;
            }
        }
        unreachable!("Stuff still got through");
    }
    let count = grid.data.iter().filter(|&x| *x == Field::Sand).count();
    assert!(count == stuck, "Count and stuck sand is equal");

    stuck
}
