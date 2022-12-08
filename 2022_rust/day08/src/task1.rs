use crate::common::Matrix;

pub fn solve(grid: &Matrix) -> isize {
    let mut visible = Matrix::new(grid.width, grid.height);

    // mark edges visible
    for i in 0..visible.width {
        let first_row_el = visible.get_mut(i, 0);
        *first_row_el = 1;
        let last_row_el = visible.get_mut(i, visible.height - 1);
        *last_row_el = 1;
    }

    for i in 0..visible.height {
        let first_col_el = visible.get_mut(0, i);
        *first_col_el = 1;
        let last_col_el = visible.get_mut(visible.width - 1, i);
        *last_col_el = 1;
    }

    // top-down & down-top
    for col in 1..grid.width - 1 {
        let mut highest = *grid.get(col, 0);
        for row in 1..grid.height - 1 {
            highest = check_visibility(grid, col, row, highest, &mut visible);
        }

        let mut highest = *grid.get(col, grid.height - 1);
        for row in (1..grid.height - 1).rev() {
            highest = check_visibility(grid, col, row, highest, &mut visible);
        }
    }

    // left-right
    for row in 1..grid.height - 1 {
        let mut highest = *grid.get(0, row);
        for col in 1..grid.width - 1 {
            highest = check_visibility(grid, col, row, highest, &mut visible);
        }

        let mut highest = *grid.get(grid.width - 1, row);
        for col in (1..grid.width - 1).rev() {
            highest = check_visibility(grid, col, row, highest, &mut visible);
        }
    }

    visible.data.iter().sum()
}

fn check_visibility(
    grid: &Matrix,
    col: usize,
    row: usize,
    highest: isize,
    visible: &mut Matrix,
) -> isize {
    let mut highest = highest;
    let tree = *grid.get(col, row);
    if tree > highest {
        highest = tree;
        *(visible.get_mut(col, row)) = 1;
    }
    highest
}
