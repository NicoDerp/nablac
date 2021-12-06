use std::fmt;
use rand;

pub struct Matrix {
	rows: usize,
	columns: usize,
	data: Vec<Vec<f32>>,
}

impl Matrix {
	pub fn new(rows: usize, columns: usize) -> Self {
		Matrix {
			rows,
			columns,
			data: vec![vec![0.0; rows]; columns],
		}
	}

	pub fn copy(&self) -> Matrix {
		let mut m = Matrix::new(self.rows, self.columns);
		for i in 0..self.rows {
			for j in 0..self.columns {
				m.data[i][j] = self.data[i][j];
			}
		}
		return m;
	}

	pub fn map(&mut self, f: &dyn Fn(f32, usize, usize) -> f32) -> &Matrix {
		for i in 0..self.rows-1 {
			for j in 0..self.columns-1 {
				self.data[i][j] = f(self.data[i][j], i, j);
			}
		}
		self
	}

	pub fn randomized(rows: usize, columns: usize) -> Matrix {
		let mut m = Matrix::new(rows, columns);
		m.map(&|_,_,_| rand::random::<f32>());
		m
	}

	pub fn add(&self, b: Matrix) {
		
	}

	pub fn multiplyMat(&self, b: Matrix) {

	}

	pub fn multiplyVec(&self, b: Vec<f32>) -> Vec<f32> {
		let mut vec = Vec::with_capacity(self.columns.try_into().unwrap());
		for i in 0..self.rows {
			let mut s = 0.0;
			for j in 0..self.columns {
				s += self.data[i][j];
			vec.push(b[i] * s);
			}
		}
		return vec;
	}
}

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "rows: {}, columns: {}, data:\n{:?}", self.rows, self.columns, self.data)
	}
}
