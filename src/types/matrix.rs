use core::panic;
use std::ops::{Index, IndexMut, Mul};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};
use crate::{eq, Tuple};

const SUBMATRIX_INDICES_3X3: [[usize; 4]; 9] = [[4, 5, 7, 8], [3, 5, 6, 8], [3, 4, 6, 7],
    [1, 2, 7, 8], [0, 2, 6, 8], [3, 4, 6, 7], [1, 2, 4, 5], [0, 2, 3, 5], [0, 1, 3, 4]];

const SUBMATRIX_INDICES_4X4: [[usize; 9]; 16] = [[5, 6, 7, 9, 10, 11, 13, 14, 15], [4, 6, 7, 8, 10, 11, 12, 14, 15],
    [4, 5, 7, 8, 9, 11, 12, 13, 15], [4, 5, 6, 8, 9, 10, 12, 13, 14], [1, 2, 3, 9, 10, 11, 13, 14, 15],[0, 2, 3, 8, 10, 11, 12, 14, 15],
    [0, 1, 3, 8, 9, 11, 12, 13, 15], [0, 1, 2, 8, 9, 10, 12, 13, 14], [1, 2, 3, 5, 6, 7, 13, 14, 15], [0, 2, 3, 4, 6, 7, 12, 14, 15],
    [0, 1, 3, 4, 5, 7, 12, 13, 15], [0, 1, 2, 4, 5, 6, 12, 13, 14], [1, 2, 3, 5, 6, 7, 9, 10, 11], [0, 2, 3, 4, 6, 7, 8, 10, 11],
    [0, 1, 3, 4, 5, 7, 8, 9, 11], [0, 1, 2, 4, 5, 6, 8, 9, 10]];

pub enum Axis {
    X,
    Y,
    Z
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    values: [f32; 16],
    size: usize,
    determinant: f32,
    cofactors: [f32; 16],
}

impl Matrix {
    pub fn new_4x4(values: [f32; 16]) -> Self {
        let mut result = Self {
            values,
            size: 4,
            determinant: 0.0,
            cofactors: [0.0; 16],
        };

        result.cofactors = result.cofactor_array();
        result.determinant = result.calc_determinant();

        result
    }

    pub fn new_3x3(values: [f32; 9]) -> Self {
        let mut padded_values = [0.0; 16];
        padded_values[..9].copy_from_slice(&values);

        let mut result = Self {
            values: padded_values,
            size: 3,
            determinant: 0.0,
            cofactors: [0.0; 16],
        };

        result.cofactors = result.cofactor_array();
        result.determinant = result.calc_determinant();

        result
    }

    pub fn new_2x2(values: [f32; 4]) -> Self {
        let mut padded_values = [0.0; 16];
        padded_values[..4].copy_from_slice(&values);

        let mut result = Self {
            values: padded_values,
            size: 2,
            determinant: 0.0,
            cofactors: [0.0; 16],
        };

        // result.cofactors = result.cofactor_array();
        result.determinant = result.calc_determinant();

        result
    }

    pub fn default(size: usize) -> Self {
        Self {
            values: [0.0; 16],
            size,
            determinant: 0.0,
            cofactors: [0.0; 16]
        }
    }

    fn cofactor_array(&self) -> [f32; 16] {
        //dbg!("cofactors calculating");

        let mut cofactors = [0.0; 16];
        
        for r in 0..self.size {
            for c in 0..self.size {
                cofactors[c * self.size + r] = self.calc_cofactor(r, c);
            }
        }

        cofactors
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Self {
        assert!(from.is_point());
        assert!(to.is_point());
        assert!(up.is_vector());

        let forward = (to - from).norm();
        let left = forward.cross(up.norm());
        let true_up = left.cross(forward);

        Self::new_4x4([
            left.x, left.y, left.z, 0.0,
            true_up.x, true_up.y, true_up.z, 0.0,
            -forward.x, -forward.y, -forward.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]) * Matrix::translation(-from.x, -from.y, -from.z)
    }

    pub fn transpose(&self) -> Self {
        Self::new_4x4([
            self.values[0], self.values[4], self.values[8], self.values[12],
            self.values[1], self.values[5], self.values[9], self.values[13],
            self.values[2], self.values[6], self.values[10], self.values[14],
            self.values[3], self.values[7], self.values[11], self.values[15]
        ])
    }

    pub fn calc_determinant(&self) -> f32 {
        if self.size == 2 {
            return self.values[0] * self.values[3] - self.values[1] * self.values[2];
        }

        let mut result = 0.0;
        for i in 0..self.size {
            result += self.values[i] * self.cofactor(0, i);
        }

        result
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        self.cofactors[row + col * self.size]
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        if self.size == 4 {
            let keep = SUBMATRIX_INDICES_4X4[row * 4 + col];
            Self::new_3x3([
                self.values[keep[0]], self.values[keep[1]], self.values[keep[2]],
                self.values[keep[3]],self.values[keep[4]], self.values[keep[5]],
                self.values[keep[6]], self.values[keep[7]],self.values[keep[8]]
            ])
        }
        else if self.size == 3 {
            let keep = SUBMATRIX_INDICES_3X3[row * 3 + col];
            Self::new_2x2([self.values[keep[0]], self.values[keep[1]], self.values[keep[2]], self.values[keep[3]]])
        }
        else {
            panic!("{} sized matrixes don't support submatrix operations", self.size);
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant
    }

    //#[inline]
    pub fn calc_cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        }
        else {
            -self.minor(row, col)
        }
    }

    pub fn invertible(&self) -> bool {
        self.determinant != 0.0
    }

    pub fn inverse(&self) -> Option<Self> {
        if !self.invertible() {
            return None;
        }

        let mut result = Self::default(4);
        let determinant = self.determinant;

        for row in 0..self.size {
            for col in 0..self.size {
                let cofactor = self.cofactor(row, col);
                result[(col, row)] = cofactor / determinant;
            }
        }
        Some(result)
    }

    pub fn identity(size: usize) -> Self {
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

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self::new_4x4([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Self::translation(x, y, z) * *self
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self::new_4x4([
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        Self::scaling(x, y, z) * *self
    }

    pub fn rotation_x(ang: f32) -> Self {
        Self::new_4x4([
            1.0, 0.0,       0.0,        0.0,
            0.0, ang.cos(), -ang.sin(), 0.0,
            0.0, ang.sin(), ang.cos(),  0.0,
            0.0, 0.0,       0.0,        1.0
        ])    
    }

    pub fn rotation_y(ang: f32) -> Self {
        Self::new_4x4([
            ang.cos(),  0.0, ang.sin(), 0.0,
            0.0,        1.0, 0.0,       0.0,
            -ang.sin(), 0.0, ang.cos(), 0.0,
            0.0,        0.0, 0.0,       1.0
        ])
    }

    pub fn rotation_z(ang: f32) -> Self {
        Self::new_4x4([
            ang.cos(), -ang.sin(), 0.0, 0.0,
            ang.sin(), ang.cos(),  0.0, 0.0,
            0.0,       0.0,        1.0, 0.0,
            0.0,       0.0,        0.0, 1.0
        ])
    }

    pub fn rotation(axis: Axis, ang: f32) -> Self {
        match axis {
            Axis::X => Self::rotation_x(ang),
            Axis::Y => Self::rotation_y(ang),
            Axis::Z => Self::rotation_z(ang),
        }
    }

    pub fn rotate_x(&self, ang: f32) -> Self {
        Self::rotation_x(ang) * *self
    }

    pub fn rotate_y(&self, ang: f32) -> Self {
        Self::rotation_y(ang) * *self
    }

    pub fn rotate_z(&self, ang: f32) -> Self {
        Self::rotation_z(ang) * *self
    }

    pub fn rotate(&self, axis: Axis, ang: f32) -> Self {
        Self::rotation(axis, ang) * *self
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Self::new_4x4([
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * *self
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

        for row in 0..self.size {
            for col in 0..self.size {
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
        let index = index.1 + index.0 * self.size;
        &self.values[index]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.1 + index.0 * self.size;
        &mut self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};
    use crate::Tuple;
    use super::{Matrix, Axis};

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

        assert_eq!(matrix.size, 4);
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

        assert_eq!(matrix.size, 2);
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

        assert_eq!(matrix.size, 3);
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
        assert_eq!(matrix.determinant, 17.0);

        let matrix = Matrix::new_3x3([
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        ]);
        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant, -196.0);

        let matrix = Matrix::new_4x4([
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);
        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant, -4071.0);
    }

    #[test]
    fn submatrix() {
        let matrix = Matrix::new_3x3([
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        ]);
        let sub = Matrix::new_2x2([
            -3.0, 2.0,
            0.0, 6.0
        ]);

        assert_eq!(matrix.submatrix(0, 2), sub);

        let matrix = Matrix::new_4x4([
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 6.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        ]);
        let sub = Matrix::new_3x3([
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        ]);

        assert_eq!(matrix.submatrix(2, 1), sub);
    }

    #[test]
    fn minor() {
        let matrix = Matrix::new_3x3(
            [
                3.0, 5.0, 0.0,
                2.0, -1.0, -7.0,
                6.0, -1.0, 5.0
            ]
        );
        let sub = matrix.submatrix(1, 0);
        assert_eq!(sub.determinant, 25.0);
        assert_eq!(matrix.minor(1, 0), 25.0)
    }

    #[test]
    fn cofactor() {
        let matrix = Matrix::new_3x3(
            [
                3.0, 5.0, 0.0,
                2.0, -1.0, -7.0,
                6.0, -1.0, 5.0
            ]
        );
        assert_eq!(matrix.minor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.minor(1, 0), 25.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }

    #[test]
    fn inverse() {
        let matrix = Matrix::new_4x4([
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0
        ]);
        assert_eq!(matrix.determinant, -2120.0);
        assert!(matrix.invertible());

        let matrix = Matrix::new_4x4([
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        ]);
        assert_eq!(matrix.determinant, 0.0);
        assert!(!matrix.invertible());

        let matrix = Matrix::new_4x4([
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        ]);
        let inverted = matrix.inverse().unwrap();
        let expected = Matrix::new_4x4([
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        ]);

        assert_eq!(matrix.determinant, 532.0);
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(inverted[(3, 2)], -160.0/532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(inverted[(2, 3)], 105.0/532.0);
        assert_eq!(inverted, expected);

        let matrix = Matrix::new_4x4([
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0
        ]);
        let inverted = matrix.inverse().unwrap();
        let expected = Matrix::new_4x4([
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692,  0.12308,  0.02564,  0.03077,
             0.35897,  0.35897,  0.43590,  0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308,
        ]);
        assert_eq!(inverted, expected);

        let matrix = Matrix::new_4x4([
             9.0,  3.0,  0.0,  9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0,  9.0,  6.0,  4.0,
            -7.0,  6.0,  6.0,  2.0,
        ]);
        let inverted = matrix.inverse().unwrap();
        let expected = Matrix::new_4x4([
            -0.04074, -0.07778,  0.14444, -0.22222,
            -0.07778,  0.03333,  0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926,  0.12963,
             0.17778,  0.06667, -0.26667,  0.33333,
        ]);
        assert_eq!(inverted, expected);

        let mat1 = Matrix::new_4x4([
             3.0, -9.0,  7.0,  3.0,
             3.0, -8.0,  2.0, -9.0,
            -4.0,  4.0,  4.0,  1.0,
            -6.0,  5.0, -1.0,  1.0,
        ]);
        let mat2 = Matrix::new_4x4([
            8.0,  2.0,  2.0,  2.0,
            3.0, -1.0,  7.0,  0.0,
            7.0,  0.0,  5.0,  4.0,
            6.0, -2.0,  0.0,  5.0,
        ]);
        let mat3 = mat1 * mat2;
        assert_eq!(mat3 * mat2.inverse().unwrap(), mat1);
    }

    #[test]
    fn combined_tranform() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rot = Matrix::rotation_x(FRAC_PI_2);
        let scale = Matrix::scaling(5.0, 5.0, 5.0);
        let translation  = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = rot * p;
        let p3 = scale * p2;
        let p4 = translation * p3;

        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));

        let transformation = translation * scale * rot;
        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));

        let transformation = Matrix::identity(4)
            .rotate_x(FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn translate() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));

        let transform = Matrix::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 7.0, 3.0));

        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, Tuple::vector(-3.0, 4.0, 5.0));
    }

    #[test]
    fn scale() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
        
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::vector(-8.0, 18.0, 32.0));

        let transform = Matrix::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let p = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::vector(-2.0, 2.0, 2.0));

        let transform = Matrix::scaling(-1.0, 1.0, 1.0).inverse().unwrap();
        let p = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::vector(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotate() {
        let half_quarter = Matrix::rotation_x(FRAC_PI_4);
        let full_quarter = Matrix::rotation_x(FRAC_PI_2);
        let p = Tuple::point(0.0, 1.0, 0.0);
        assert_eq!(half_quarter * p, Tuple::point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(half_quarter.inverse().unwrap() * p, Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));

        let full_quarter = Matrix::rotation(Axis::X, FRAC_PI_2);
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));

        let half_quarter = Matrix::rotation_y(FRAC_PI_4);
        let full_quarter = Matrix::rotation_y(FRAC_PI_2);
        let p = Tuple::point(0.0, 0.0, 1.0);
        assert_eq!(half_quarter * p, Tuple::point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));

        let full_quarter = Matrix::rotation(Axis::Y, FRAC_PI_2);
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));

        let half_quarter = Matrix::rotation_z(FRAC_PI_4);
        let full_quarter = Matrix::rotation_z(FRAC_PI_2);
        let p = Tuple::point(0.0, 1.0, 0.0);
        assert_eq!(half_quarter * p, Tuple::point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));

        let full_quarter = Matrix::rotation(Axis::Z, FRAC_PI_2);
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear() {
        let point = Tuple::point(2.0, 3.0, 4.0);

        let shear = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(shear * point, Tuple::point(5.0, 3.0, 4.0));

        let shear = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(shear * point, Tuple::point(6.0, 3.0, 4.0));

        let shear = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(shear * point, Tuple::point(2.0, 5.0, 4.0));

        let shear = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(shear * point, Tuple::point(2.0, 7.0, 4.0));

        let shear = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(shear * point, Tuple::point(2.0, 3.0, 6.0));

        let shear = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(shear * point, Tuple::point(2.0, 3.0, 7.0));

        let shear = Matrix::shearing(2.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(shear * point, Tuple::point(8.0, 3.0, 7.0));
    }

    #[test]
    fn view_transform() {
        // default orientation is no change
        let t = Matrix::view_transform(Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, -1.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(t, Matrix::identity(4));

        // looking backwards reflects
        let t = Matrix::view_transform(Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, 1.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));

        // we move the world, not the eye
        let t = Matrix::view_transform(Tuple::point(0.0, 0.0, 8.0),
            Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));

        // some random numbers
        let t = Matrix::view_transform(Tuple::point(1.0, 3.0, 2.0),
            Tuple::point(4.0, -2.0, 8.0), Tuple::vector(1.0, 1.0, 0.0));
        assert_eq!(t, Matrix::new_4x4(
            [
                -0.50709, 0.50709, 0.67612, -2.36643,
                0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]));
    }
}