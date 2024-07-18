use uuid::Uuid;

use crate::{types::ray::Ray, Tuple, types::intersection::Intersection, Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrix::identity(4),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray = ray.transform(self.transform.inverse().unwrap());

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

    pub fn normal(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());
        let point = self.transform.inverse().unwrap() * point;
        let mut normal = point - Tuple::point(0.0, 0.0, 0.0);
        normal *= self.transform.inverse().unwrap().transpose();
        normal.w = 0.0;
        normal.norm()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{PI, SQRT_2};

    use super::Sphere;
    use crate::{Matrix, Tuple};
    use crate::types::ray::Ray;

    #[test]
    fn new() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));
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

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let inters = s.intersect(r);
        dbg!(&inters);
        assert_eq!(inters.len(), 2);
        assert_eq!(inters[0].t, 3.0);
        assert_eq!(inters[1].t, 7.0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Matrix::translation(5.0, 0.0, 0.0);
        let inters = s.intersect(r);
        assert_eq!(inters.len(), 0);
    }

    #[test]
    fn normal() {
        let s = Sphere::new();
        assert_eq!(s.normal(Tuple::point(1.0, 0.0, 0.0)), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(s.normal(Tuple::point(0.0, 1.0, 0.0)), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(s.normal(Tuple::point(0.0, 0.0, 1.0)), Tuple::vector(0.0, 0.0, 1.0));

        let root_3_over_3 = 3.0_f32.sqrt() / 3.0;
        assert_eq!(s.normal(Tuple::point(root_3_over_3, root_3_over_3, root_3_over_3)),
            Tuple::vector(root_3_over_3, root_3_over_3, root_3_over_3));

        let n = s.normal(Tuple::point(root_3_over_3, root_3_over_3, root_3_over_3));
        assert_eq!(n, n.norm());

        let mut s = Sphere::new();
        s.transform = Matrix::translation(0.0, 1.0, 0.0);
        assert_eq!(s.normal(Tuple::point(0.0, 1.70711, -0.70711)), Tuple::vector(0.0, 0.70711, -0.70711));

        let mut s = Sphere::new();
        s.transform = Matrix::rotation_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        assert_eq!(s.normal(Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0)), Tuple::vector(0.0, 0.97014, -0.24254));

    }
}
