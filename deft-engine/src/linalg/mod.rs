use std::vec::Vec;

pub struct Matrix {
    pub rows: u32,
    pub cols: u32,
    pub data: Vec<u32>
}

impl Matrix {
    pub fn zeros(rows: u32, cols: u32) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0; (rows * cols) as usize]
        }
    }

    pub fn ones(rows: u32, cols: u32) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![1; (rows * cols) as usize]
        }
    }

    pub fn from_data(rows: u32, cols: u32, data: Vec<u32>) -> Matrix {
        Matrix {
            rows,
            cols,
            data
        }
    }

    pub fn print(data: &Matrix) {
        for i in 0..data.rows {
            for j in 0..data.cols {
                print!("{} ", data.get(i, j));
            }
            println!();
        }
    }

    pub fn get(&self, row: u32, col: u32) -> u32 {
        self.data[(row * self.cols + col) as usize]
    }

    pub fn set(&mut self, row: u32, col: u32, value: u32) {
        self.data[(row * self.cols + col) as usize] = value;
    }

    pub fn get_block(&self, start_row: u32, start_col: u32, end_row: u32, end_col: u32) -> Matrix {
        let mut output = Matrix::zeros(end_row - start_row, end_col - start_col);
        for i in start_row..end_row {
            for j in start_col..end_col {
                output.set(i - start_row, j - start_col, self.get(i, j));
            }
        }
        output
    }

    pub fn add_block(&mut self, start_row: u32, start_col: u32, block: &Matrix) {
        for i in 0..block.rows {
            for j in 0..block.cols {
                self.set(i + start_row, j + start_col, self.get(i + start_row, j + start_col) + block.get(i, j));
            }
        }
    }

    pub fn set_block(&mut self, start_row: u32, start_col: u32, block: &Matrix) {
        for i in 0..block.rows {
            for j in 0..block.cols {
                self.set(i + start_row, j + start_col, block.get(i, j));
            }
        }
    }
    pub fn overlay_block(&mut self, start_row: u32, start_col: u32, block: &Matrix) {
        for i in 0..block.rows {
            for j in 0..block.cols {
                if block.get(i, j) != 0 {
                    self.set(i + start_row, j + start_col, block.get(i, j));
                }
            }
        }
    }

    pub fn get_row(&self, row: u32) -> Matrix {
        let mut output = Matrix::zeros(1, self.cols);
        for i in 0..self.cols {
            output.set(0, i, self.get(row, i));
        }
        output
    }

    pub fn get_col(&self, col: u32) -> Matrix {
        let mut output = Matrix::zeros(self.rows, 1);
        for i in 0..self.rows {
            output.set(i, 0, self.get(i, col));
        }
        output
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_data = vec![0; (self.rows * self.cols) as usize];
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_data[(j * self.rows + i) as usize] = self.data[(i * self.cols + j) as usize];
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: new_data
        }
    }

    pub fn add(a: &Matrix, b: &Matrix) -> Matrix {
        if a.rows != b.rows || a.cols != b.cols {
            panic!("Matrix dimensions must match for addition");
        }
        let mut output = Matrix::zeros(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                output.set(i, j, a.get(i, j) + b.get(i, j));
            }
        }
        output
    }

    pub fn sub(a: &Matrix, b: &Matrix) -> Matrix {
        if a.rows != b.rows || a.cols != b.cols {
            panic!("Matrix dimensions must match for subtraction");
        }
        let mut output = Matrix::zeros(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                output.set(i, j, a.get(i, j) - b.get(i, j));
            }
        }
        output
    }

    pub fn mul(a: &Matrix, b: &Matrix) -> Matrix {
        if a.cols != b.rows {
            panic!("Matrix dimensions must match for multiplication");
        }
        let mut output = Matrix::zeros(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                output.set(i, j, a.get(i, j) * b.get(i, j));
            }
        }
        output
    }

    pub fn vector_dot(a: Matrix, b: Matrix) -> u32 {
        if a.rows != 1 || b.rows != 1 {
            panic!("Vector dot product requires two vectors");
        }
        if a.cols != b.cols {
            panic!("Vector dot product requires vectors of the same length");
        }
        let mut sum = 0;
        for i in 0..a.cols {
            sum += a.get(0, i) * b.get(0, i);
        }
        sum
    }

    pub fn dot(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Matrix dimensions must match for dot product");
        }
        let mut new_data = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0;
                let row = self.get_row(i);
                let col = other.get_col(j);
                sum += Matrix::vector_dot(row, col.transpose()) as u32;
                new_data.set(i, j, sum);
            }
        }
        new_data
    }

    pub fn flatten(&self) -> Vec<u32> {
        self.data.clone()
    }
    
}