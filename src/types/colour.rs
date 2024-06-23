use std::ops::Mul;
use derive_more::{Add, Sub, AddAssign, SubAssign};
use super::eq;

#[derive(Debug, Clone, Copy, Add, Sub, AddAssign, SubAssign)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        eq(self.r, other.r) &&
        eq(self.g, other.g) &&
        eq(self.b, other.b)
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, factor: f32) -> Self::Output {
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, other: Colour) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let col = Colour::new(-0.5, 0.4, 1.7);
        assert_eq!(col.r, -0.5);
        assert_eq!(col.g, 0.4);
        assert_eq!(col.b, 1.7);
    }

    #[test]
    fn equal() {
        let col1 = Colour::new(-0.5, 0.4, 1.7);
        let col2 = Colour::new(-0.5, 0.4, 1.7);
        assert_eq!(col1, col2);
    
        let col1 = Colour::new(-0.5, 0.4, 1.7);
        let col2 = Colour::new(0.5, 7.4, -8.7);
        assert_ne!(col1, col2);
    }

    #[test]
    fn add() {
        let col1 = Colour::new(0.9, 0.6, 0.75);
        let col2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(col1 + col2, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub() {
        let col1 = Colour::new(0.9, 0.6, 0.75);
        let col2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(col1 - col2, Colour::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_scalar() {
        let col1 = Colour::new(0.9, 0.6, 0.75);
        let col2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(col1 - col2, Colour::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_blend() {
        let col1 = Colour::new(1.0, 0.2, 0.4);
        let col2 = Colour::new(0.9, 1.0, 0.1);
        assert_eq!(col1 * col2, Colour::new(0.9, 0.2, 0.04));
    }
}