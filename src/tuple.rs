use std::ops::Mul;

use derive_more::{Add, Sub, Neg};
use super::eq;

#[derive(Debug, Add, Sub, Neg)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }

    fn vector(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
        }
    }

    fn point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    fn is_point(&self) -> bool {
        return self.w == 1.0;
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
}