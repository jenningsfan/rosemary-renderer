use crate::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(origin.is_point(), "Origin must be a point");
        assert!(direction.is_vector(), "Direction must be a vector");

        Self {
            origin,
            direction,
        }
    }

    pub fn position(&self, distance: f32) -> Tuple {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
mod tests {
    use crate::Tuple;
    use super::Ray;

    #[test]
    fn new() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn position() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0),
            Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}