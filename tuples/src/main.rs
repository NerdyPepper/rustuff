use std::fmt;

fn main() {
	let tup1 = (2i32, true);
	let tup2 = rev(tup1);
	println!("{:?}", tup2);

	let a = Matrix (0.0, 0.1, 1.0, 1.1);

	println!("{}", a);
	println!("\nTranspose:");
	println!("{}", transpose(a));
}

fn rev(pair: (i32, bool)) -> (bool, i32) {
	(pair.1, pair.0)
}

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "( {:.1} {:.1} )\n", self.0, self.1);
		write!(f, "( {:.1} {:.1} )", self.2, self.3)
	}
}

fn transpose(matrix: Matrix) -> Matrix {
	let trans = Matrix (matrix.0, matrix.2, matrix.1, matrix.3);
	return trans
}
