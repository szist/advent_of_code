#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Field {
    Air = 0,
    Stone = 1,
    Sand = 2,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Field>,
}

// Pos(col, row)
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![Field::Air; width * height],
        }
    }
    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut Field {
        assert!(col < self.width && row < self.height, "({}, {})", col, row);
        self.data.get_mut(row * self.width + col).unwrap()
    }

    pub fn get(&self, col: usize, row: usize) -> &Field {
        assert!(col < self.width && row < self.height);
        self.data.get(row * self.width + col).unwrap()
    }

    pub fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let p = match &self.get(col, row) {
                    Field::Air => '.',
                    Field::Stone => '#',
                    Field::Sand => 'o',
                };
                print!("{p}")
            }
            println!("");
        }
    }
}

pub fn parse_input_lines(input: &str) -> (Matrix, usize) {
    let mut min: Pos = Pos(500, 0);
    let mut max: Pos = Pos(500, 0);
    let walls: Vec<Vec<Pos>> = input
        .trim_end()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (col, row) = point.split_once(",").unwrap();
                    Pos(col.parse().unwrap(), row.parse().unwrap())
                })
                .collect::<Vec<Pos>>()
        })
        .collect();
    walls.iter().flatten().for_each(|x| {
        min.0 = x.0.min(min.0);
        min.1 = x.1.min(min.1);
        max.0 = x.0.max(max.0);
        max.1 = x.1.max(max.1);
    });

    let height = max.1 - min.1 + 3; // last line - if send gets here, it falls off
    let col_offset = min.0 - 2 - height;
    let width = max.0 - min.0 + 5 + 2 * height; // double edges
    println!("{:?} {:?} {} {}", min, max, width, height);

    let mut grid = Matrix::new(width as usize, height as usize);

    walls.iter().for_each(|wall| {
        for i in 1..wall.len() {
            let mut cols = vec![wall[i - 1].0, wall[i].0];
            cols.sort();
            let mut rows = vec![wall[i - 1].1, wall[i].1];
            rows.sort();
            for col in cols[0]..=cols[1] {
                for row in rows[0]..=rows[1] {
                    let field = grid.get_mut(
                        (col - col_offset).try_into().unwrap(),
                        row.try_into().unwrap(),
                    );
                    *field = Field::Stone;
                }
            }
        }
    });
    (grid, 500 - col_offset as usize)
}

pub fn drop_sand(grid: &Matrix, from: usize) -> Option<Pos> {
    let mut col = from;
    for h in 1..grid.height {
        let left = *grid.get(col - 1, h);
        let center = *grid.get(col, h);
        let right = *grid.get(col + 1, h);

        //println!("Dropping at {h} {col}");

        if center == Field::Air {
            //pass
        } else if left == Field::Air {
            col -= 1;
        } else if right == Field::Air {
            col += 1;
        } else {
            return Some(Pos(col, h - 1));
        }
    }
    return None;
}
