// I don't like that it's all pub, but oh well
// it works tm
#[derive(PartialEq, Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<isize>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![0; width * height],
        }
    }
    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut isize {
        assert!(col < self.width && row < self.height);
        self.data.get_mut(row * self.width + col).unwrap()
    }

    pub fn get(&self, col: usize, row: usize) -> &isize {
        assert!(col < self.width && row < self.height);
        self.data.get(row * self.width + col).unwrap()
    }
}

pub fn parse_input_matrix(input: &str) -> Matrix {
    let lines = input.split('\n');
    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let data = lines
        .flat_map(|line| line.as_bytes())
        .map(|el| (el - b'0') as isize)
        .collect::<Vec<isize>>();
    Matrix {
        width,
        height,
        data,
    }
}
