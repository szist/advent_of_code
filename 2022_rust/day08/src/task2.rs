use crate::common::Matrix;

fn get_score(grid: &Matrix, row: usize, col: usize) -> usize {
    let mut above = 0;
    let mut below = 0;
    let mut right = 0;
    let mut left = 0;

    let height = grid.get(col, row);
    //check above
    for r in (0..row).rev() {
        above = row - r;
        if grid.get(col, r) >= height {
            break;
        }
    }

    //check below
    for r in row + 1..grid.height {
        below = r - row;
        if grid.get(col, r) >= height {
            break;
        }
    }

    //check right
    for c in col + 1..grid.width {
        right = c - col;
        if grid.get(c, row) >= height {
            break;
        }
    }

    // check left
    for c in (0..col).rev() {
        left = col - c;
        if grid.get(c, row) >= height {
            break;
        }
    }

    above * below * right * left
}

pub fn solve(grid: &Matrix) -> usize {
    let mut max = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            let score = get_score(grid, row, col);
            if score > max {
                max = score;
            }
        }
    }
    max
}
