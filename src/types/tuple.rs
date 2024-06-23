use std::ops::{Mul, Div, MulAssign, DivAssign};

use derive_more::{Add, Sub, Neg, AddAssign, SubAssign};
use super::eq;

#[derive(Debug, Clone, Copy, Add, Sub, Neg, AddAssign, SubAssign)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
        }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn mul_scalar(&self, factor: f32) -> Self {
        *self * factor
    }

    pub fn dot(&self, other: Self) -> f32 {
        *self * other
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        eq(self.x, other.x) &&
        eq(self.y, other.y) &&
        eq(self.z, other.z) &&
        eq(self.w, other.w)
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, factor: f32) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,            
        }
    }
}

impl Mul<Tuple> for Tuple {
    type Output = f32;

    fn mul(self, other: Tuple) -> Self::Output {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, factor: f32) -> Self::Output {
        Self {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
            w: self.w / factor,            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);

        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);

        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn is_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(tuple.is_point());

        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!tuple.is_point());
    }

    #[test]
    fn is_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(!tuple.is_vector());

        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(tuple.is_vector());
    }

    #[test]
    fn equal() {
        let tuple1 = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let tuple2 = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple1, tuple2);
    
        let tuple1 = Tuple::new(3.3, -2.2, 0.1, 0.0);
        let tuple2 = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_ne!(tuple1, tuple2);
    }

    #[test]
    fn add() {
        let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let result = Tuple::new(1.0, 1.0, 6.0, 1.0);
        assert_eq!(tuple1 + tuple2, result);
    }

    #[test]
    fn sub() {
        let tuple1 = Tuple::point(3.0, 2.0, 1.0);
        let tuple2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(tuple1 - tuple2, Tuple::vector(-2.0, -4.0, -6.0));

        let tuple1 = Tuple::point(3.0, 2.0, 1.0);
        let tuple2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(tuple1 - tuple2, Tuple::point(-2.0, -4.0, -6.0));

        let tuple1 = Tuple::vector(3.0, 2.0, 1.0);
        let tuple2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(tuple1 - tuple2, Tuple::vector(-2.0, -4.0, -6.0));
    
        let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
        let tuple2 = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(tuple1 - tuple2, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn neg() {
        let tuple = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(-tuple, Tuple::vector(-1.0, 2.0, -3.0));

        let tuple = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(-tuple, Tuple::new(1.0, -2.0, 3.0, -4.0));
    }

    #[test]
    fn mul_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));

        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn div_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude() {
        let vector = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);

        let vector = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);

        let vector = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(vector.magnitude(), 1.0);

        let vector = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(vector.magnitude(), 14.0_f32.sqrt());

        let vector = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(vector.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn norm() {
        let vector = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(vector.norm(), Tuple::vector(1.0, 0.0, 0.0));

        let vector = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(vector.norm(), Tuple::vector(0.26726, 0.53452, 0.80178));
        assert!(eq(vector.norm().magnitude(), 1.0));
    }

    #[test]
    fn dot() {
        let vector1 = Tuple::vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(vector1 * vector2, 20.0);
    }

    #[test]
    fn cross() {
        let vector1 = Tuple::vector(1.0, 2.0, 3.0);
        let vector2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(vector1.cross(vector2), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(vector2.cross(vector1), Tuple::vector(1.0, -2.0, 1.0));
    }
}