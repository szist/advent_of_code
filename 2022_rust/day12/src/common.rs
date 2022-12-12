// I don't like that it's all pub, but oh well
// it works tm
#[derive(PartialEq, Debug, Eq)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<isize>,
}

impl Matrix {
    pub fn get(&self, col: usize, row: usize) -> &isize {
        assert!(
            col < self.width && row < self.height,
            "col: {}, row: {} ({}x{})",
            col,
            row,
            self.width,
            self.height
        );
        self.data.get(row * self.width + col).unwrap()
    }
}

pub const BASE: isize = b'a' as isize;
pub const START: isize = b'S' as isize - b'a' as isize;
pub const END: isize = b'E' as isize - b'a' as isize;

pub fn parse_input_matrix(input: &str) -> Matrix {
    let lines = input.split('\n');
    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let data = lines
        .flat_map(|line| line.as_bytes())
        .map(|&el| el as isize - BASE)
        .collect::<Vec<isize>>();
    Matrix {
        width,
        height,
        data,
    }
}

pub fn get_height(val: &isize) -> isize {
    match *val {
        START => 0,
        END => b'z' as isize - BASE,
        _ => *val,
    }
}
