use crate::EPSILON;
use super::{ray::Ray, sphere::Sphere, tuple::Tuple};

pub struct IntersectionComps<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eye: Tuple,
    pub normal: Tuple,
    pub inside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: &'a Sphere
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, obj: &'a Sphere) -> Self {
        Self {
            t,
            obj
        }
    }

    pub fn hit<'b>(inters: &'b Vec<Intersection>) -> Option<Intersection<'b>> {
        let mut min_t = f32::MAX;
        let mut min_inter = None;
    
        for i in inters {
            if i.t > 0.0 && i.t < min_t {
                min_t = i.t;
                min_inter = Some(i);
            }
        }
    
        min_inter.copied()
    }

    pub fn comps(&self, ray: &Ray) -> IntersectionComps {
        let point = ray.position(self.t);

        let mut comps = IntersectionComps {
            t: self.t,
            obj: self.obj,
            over_point: point,
            point,
            eye: -ray.direction,
            normal: self.obj.normal(point),
            inside: false,
        };
        
        if comps.normal.dot(comps.eye) < 0.0 {
            comps.inside = true;
            comps.normal = -comps.normal; // invert normal if inside
        }
        
        comps.over_point += comps.normal * EPSILON * 20.0;
        
        comps
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::{material::Material, sphere::Sphere}, Matrix, Tuple};
    use super::*;

    #[test]
    fn new() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.obj, s);

        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let inters = vec![i1, i2];
        let inter = Intersection::hit(&inters);
        assert_eq!(inter.unwrap(), i1);

        let i1 = Intersection::new(-1.0, &s);
        let i2: Intersection = Intersection::new(1.0, &s);
        let inters = vec![i1, i2];
        let inter = Intersection::hit(&inters);
        assert_eq!(inter.unwrap(), i2);

        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let inters = vec![i1, i2];
        let inter = Intersection::hit(&inters);
        assert_eq!(inter, None);

        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let inters = vec![i1, i2, i3, i4];
        let inter = Intersection::hit(&inters);
        assert_eq!(inter.unwrap(), i4);
    }

    #[test]
    fn comps_outisde() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(4.0, &s);
        let comps = i.comps(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(*comps.obj, s);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn comps_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(1.0, &s);
        let comps = i.comps(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(*comps.obj, s);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Tuple::vector(0.0, 0.0, -1.0)); // would've been (0.0, 0.0, 1.0) if outside but inside so inverted
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn shadow_compensation() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(Matrix::translation(0.0, 0.0, 1.0), Material::default());
        let i = Intersection::new(5.0, &s);
        let comps = i.comps(&r);
        assert!(comps.over_point.z < -crate::EPSILON/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}