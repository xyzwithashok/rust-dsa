use std::fmt::Debug;
use core::ops::{Add, Sub};

pub struct Matrix {
    pub matrix: Vec<Vec<i32>>,
    pub row: usize,
    pub col: usize,
}


impl Matrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        for row in matrix.iter() {
            if row.len() != matrix[0].len() {
                panic!("Invalid matrix");
            }
        }

        let row = matrix.len();
        let col = matrix[0].len();
        Matrix { matrix, row, col }
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("Matrix ({} x {})\n", self.row, self.col));
        s.push_str("[");
        for i in 0..self.row {
            s.push_str("[ ");
            for j in 0..self.col {
                s.push_str(&self.matrix[i][j].to_string());
                s.push_str(" ");
            }
            s.push_str("]");
            if i != self.row - 1 {
                s.push_str(",\n");
            }
        }
        s.push_str("]\n");
        write!(f, "{}", s)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("Invalid matrix addition");
        }

        let mut result = vec![vec![0; self.col]; self.row];
        for i in 0..self.row {
            for j in 0..self.col {
                result[i][j] = self.matrix[i][j] + other.matrix[i][j];
            }
        }

        Matrix {
            matrix: result,
            row: self.row,
            col: self.col,
        }
    }
}

impl Add<i32> for Matrix {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let mut result = vec![vec![0; self.col]; self.row];
        for i in 0..self.row {
            for j in 0..self.col {
                result[i][j] = self.matrix[i][j] + other;
            }
        }

        Matrix {
            matrix: result,
            row: self.row,
            col: self.col,
        }
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("Invalid matrix subtraction");
        }

        let mut result = vec![vec![0; self.col]; self.row];
        for i in 0..self.row {
            for j in 0..self.col {
                result[i][j] = self.matrix[i][j] - other.matrix[i][j];
            }
        }

        Matrix {
            matrix: result,
            row: self.row,
            col: self.col,
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix {
            matrix: vec![vec![0]],
            row: 1,
            col: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let m = Matrix::new(matrix);
        assert_eq!(m.row, 3);
        assert_eq!(m.col, 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]];
        let _ = Matrix::new(matrix);
    }

    #[test]
    fn test_default_matrix() {
        let m = Matrix::default();
        assert_eq!(m.row, 1);
        assert_eq!(m.col, 1);
    }

    #[test]
    fn test_debug() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let m = Matrix::new(matrix);
        println!("{:?}", m);
    }

    #[test]
    fn test_add() {
        let matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let matrix2 = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        let m1 = Matrix::new(matrix1);
        let m2 = Matrix::new(matrix2);
        let m3 = m1 + m2;
        println!("{:?}", m3);
    }

    #[test]
    fn test_invalid_add() {
        let matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let m1 = Matrix::new(matrix1);
        let m2 = m1 + 1i32;
        println!("{:?}", m2);
    }
}