use std::{iter::Cycle, slice::Iter};

pub struct Shape([[usize; 4]; 4], usize);

pub const SHAPES: [Shape; 5] = [
    // -
    Shape([[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], 1),
    // +
    Shape([[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]], 3),
    // _|
    Shape([[0, 0, 1, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]], 3),
    // |
    Shape([[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]], 4),
    // []
    Shape([[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], 2),
];

pub const WIDTH: usize = 7;

fn is_colliding(chamber: &Vec<Vec<usize>>, shape: &Shape, srow: &usize, scol: &usize) -> bool {
    for row in 0..4 {
        for col in 0..4 {
            if shape.0[row][col] > 0 {
                let curr_row = srow + row;
                let curr_col = scol + col;
                // check for out of bounds
                if curr_row >= chamber.len() || curr_col >= WIDTH {
                    return true;
                }
                // check already set
                if chamber[curr_row][curr_col] > 0 {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn drop(tetris: &mut Vec<Vec<usize>>, shape: &Shape, gust: &mut Cycle<Iter<u8>>) {
    // fill in lines if needed
    let empty_lines = tetris
        .iter()
        .position(|l| l.iter().any(|&x| x != 0))
        .unwrap();
    let needed_lines = shape.1 + 3;
    let mut row = if needed_lines > empty_lines {
        for _ in empty_lines..needed_lines {
            tetris.insert(0, vec![0; WIDTH]);
        }
        0
    } else {
        empty_lines - needed_lines
    };
    let mut col = 2;

    loop {
        let next_col = match gust.next().unwrap() {
            b'>' => col + 1,
            b'<' => {
                if col > 0 {
                    col - 1
                } else {
                    col
                }
            }
            _ => unreachable!("Bad gust direction"),
        };
        // gust moving the block left/right
        if !is_colliding(tetris, shape, &row, &next_col) {
            col = next_col;
        }

        // moving one down
        if !is_colliding(tetris, shape, &(row + 1), &col) {
            row += 1;
        } else {
            // write down the shape into the arr
            for r in 0..4 {
                for c in 0..4 {
                    if shape.0[r][c] > 0 {
                        tetris[row + r][col + c] = 1;
                    }
                }
            }
            break;
        }
    }
}
