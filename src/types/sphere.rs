use uuid::Uuid;

use crate::{types::ray::Ray, Tuple, types::intersection::Intersection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sphere {
    id: Uuid
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_ray_vec = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_ray_vec);
        let c = sphere_ray_vec.dot(sphere_ray_vec) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, *self), Intersection::new(t2, *self)]
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::Tuple;
    use crate::types::ray::Ray;

    #[test]
    fn new() {
        let s = Sphere::new();
    }

    #[test]
    fn intersect() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let inters = s.intersect(r);
        assert_eq!(inters[0].t, 4.0);
        assert_eq!(inters[1].t, 6.0);
        assert_eq!(inters[0].obj, s);
        assert_eq!(inters[1].obj, s);

        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let inters = s.intersect(r);
        assert_eq!(inters.len(), 2);
        assert_eq!(inters[0].t, 5.0);
        assert_eq!(inters[1].t, 5.0);

        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let inters = s.intersect(r);
        assert_eq!(inters.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let inters = s.intersect(r);
        assert_eq!(inters.len(), 2);
        assert_eq!(inters[0].t, -1.0);
        assert_eq!(inters[1].t, 1.0);

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let inters = s.intersect(r);
        assert_eq!(inters.len(), 2);
        assert_eq!(inters[0].t, -6.0);
        assert_eq!(inters[1].t, -4.0);
    }
}
