use std::collections::HashMap;
use std::fmt;

pub struct Matrix {
	rows: i64,
	columns: i64,
	data: HashMap<i32, i32>,
}

impl Matrix {
	pub fn new(rows: i32, columns: i32) -> Self {
		Matrix {
			rows,
			columns,
			data: HashMap::new(),
		}
	}

	pub fn copy(&self) -> Matrix {
		let m = Matrix::new(self.rows, self.columns);
		for i in 0..self.rows:
			for j in 0..self.columns:
				m.data[i][j] = self.data[i][j];
	}

	pub fn map(&self, f: &dyn Fn(f32) -> f32) {
		for i in 0..self.rows:
			for j in 0..self.columns:
				self.data[i][j] = f(self.data[i][j])
	}

	pub fn add(a: Matrix, b: Matrix) -> Matrix {
		
	}
}

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "rows: {}, columns: {}", self.rows, self.columns)
	}
}
