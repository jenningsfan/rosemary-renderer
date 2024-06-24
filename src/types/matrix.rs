use core::panic;
use std::ops::{Index, IndexMut, Mul};
use crate::{eq, Tuple};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    values: [f32; 16],
    size: u8,
}

impl Matrix {
    pub fn new_4x4(values: [f32; 16]) -> Self {
        Self {
            values,
            size: 4
        }
    }

    pub fn new_3x3(values: [f32; 9]) -> Self {
        let mut padded_values = [0.0; 16];
        padded_values[..9].copy_from_slice(&values);

        Self {
            values: padded_values,
            size: 3
        }
    }

    pub fn new_2x2(values: [f32; 4]) -> Self {
        let mut padded_values = [0.0; 16];
        padded_values[..4].copy_from_slice(&values);

        Self {
            values: padded_values,
            size: 2
        }
    }

    pub fn default(size: u8) -> Self {
        Self {
            values: [0.0; 16],
            size
        }
    }

    pub fn identity(size: u8) -> Self {
        if size != 4 {
            panic!("Only 4x4 identity matrices supported");
        }

        Self::new_4x4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn transpose(&self) -> Self {
        Self::new_4x4([
            self.values[0], self.values[4], self.values[8], self.values[12],
            self.values[1], self.values[5], self.values[9], self.values[13],
            self.values[2], self.values[6], self.values[10], self.values[14],
            self.values[3], self.values[7], self.values[11], self.values[15]
        ])
    }

    pub fn determinant(&self) -> f32 {
        if self.size == 2 {
            return self.values[0] * self.values[3] - self.values[1] * self.values[2];
        }

        0.0
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for (a, b) in self.values.iter().zip(other.values.iter()) {
            if !eq(*a, *b) {
                return false;
            }
        }

        true
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        let mut result = Matrix::default(self.size);

        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                result[(row, col)] =
                    self[(row, 0)] * other[(0, col)] + 
                    self[(row, 1)] * other[(1, col)] + 
                    self[(row, 2)] * other[(2, col)] + 
                    self[(row, 3)] * other[(3, col)]
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        let x = self[(0, 0)] * other.x + self[(0, 1)] * other.y + self[(0, 2)] * other.z + self[(0, 3)] * other.w;
        let y = self[(1, 0)] * other.x + self[(1, 1)] * other.y + self[(1, 2)] * other.z + self[(1, 3)] * other.w;
        let z = self[(2, 0)] * other.x + self[(2, 1)] * other.y + self[(2, 2)] * other.z + self[(2, 3)] * other.w;
        let w = self[(3, 0)] * other.x + self[(3, 1)] * other.y + self[(3, 2)] * other.z + self[(3, 3)] * other.w;
        Tuple::new(x, y, z, w)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.1 + index.0 * self.size as usize;
        &self.values[index]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.1 + index.0 * self.size as usize;
        &mut self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::Tuple;

    use super::Matrix;

    #[test]
    fn new() {
        let matrix = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5
            ]
        );

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);

        let matrix = Matrix::new_2x2(
            [
                -3.0, 5.0,
                1.0, -2.0,
            ]
        );

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(0, 1)], 5.0);
        assert_eq!(matrix[(1, 0)], 1.0);
        assert_eq!(matrix[(1, 1)], -2.0);

        let matrix = Matrix::new_3x3(
            [
                -3.0, 5.0, 0.0,
                1.0, -2.0, -7.0,
                0.0, 1.0, 1.0
            ]
        );

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(1, 1)], -2.0);
        assert_eq!(matrix[(2, 2)], 1.0);
    }

    #[test]
    fn eq() {
        let matrix1 = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 7.0,
                5.0, 4.0, 3.0, 2.0,
            ]
        );

        let matrix2 = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 7.0,
                5.0, 4.0, 3.0, 2.0,
            ]
        );

        assert_eq!(matrix1, matrix2);

        let matrix1 = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 7.0,
                5.0, 4.0, 3.0, 2.0,
            ]
        );

        let matrix2 = Matrix::new_4x4(
            [
                2.0, 3.0, 4.0, 5.0,
                6.0, 7.0, 8.0, 9.0,
                8.0, 7.0, 6.0, 5.0,
                4.0, 3.0, 2.0, 1.0,
            ]
        );

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn mul() {
        let matrix1 = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 6.0,
                5.0, 4.0, 3.0, 2.0,
            ]
        );

        let matrix2 = Matrix::new_4x4(
            [
                -2.0, 1.0, 2.0, 3.0,
                3.0, 2.0, 1.0, -1.0,
                4.0, 3.0, 6.0, 5.0,
                1.0, 2.0, 7.0, 8.0,
            ]
        );

        let result = Matrix::new_4x4(
            [
                20.0, 22.0, 50.0, 48.0,
                44.0, 54.0, 114.0, 108.0,
                40.0, 58.0, 110.0, 102.0,
                16.0, 26.0, 46.0, 42.0,
            ]
        );

        assert_eq!(matrix1 * matrix2, result);
        
        let matrix = Matrix::new_4x4(
            [
                1.0, 2.0, 3.0, 4.0,
                2.0, 4.0, 4.0, 2.0,
                8.0, 6.0, 4.0, 1.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        );
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);
        assert_eq!(matrix * tuple, result);
    }

    #[test]
    fn identity() {
        let matrix = Matrix::new_4x4(
            [
                0.0, 1.0, 2.0, 4.0,
                1.0, 2.0, 4.0, 8.0,
                2.0, 4.0, 8.0, 16.0,
                4.0, 8.0, 16.0, 32.0,
            ]
        );
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(Matrix::identity(4) * matrix, matrix);
        assert_eq!(Matrix::identity(4) * tuple, tuple);
    }

    #[test]
    fn transpose() {
        let matrix = Matrix::new_4x4(
            [
                0.0, 9.0, 3.0, 0.0,
                9.0, 8.0, 0.0, 8.0,
                1.0, 8.0, 5.0, 3.0,
                0.0, 0.0, 5.0, 8.0
            ]
        );
        let transposed = Matrix::new_4x4(
            [
                0.0, 9.0, 1.0, 0.0,
                9.0, 8.0, 8.0, 0.0,
                3.0, 0.0, 5.0, 5.0,
                0.0, 8.0, 3.0, 8.0
            ]
        );

        assert_eq!(matrix.transpose(), transposed);
    }

    #[test]
    fn determinant() {
        let matrix = Matrix::new_2x2([
            1.0, 5.0,
            -3.0, 2.0
        ]);
        assert_eq!(matrix.determinant(), 17.0);
    }
}