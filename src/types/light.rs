use crate::types::{colour::Colour, tuple::Tuple};

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub intensity: Colour,
    pub pos: Tuple
}

impl PointLight {
    pub fn new(intensity: Colour, pos: Tuple) -> Self {
        assert!(pos.is_point());

        Self {
            intensity,
            pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let col = Colour::new(1.0, 1.0, 1.0);
        let pos = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(col, pos);
        assert_eq!(light.intensity, col);
        assert_eq!(light.pos, pos);

    }
}