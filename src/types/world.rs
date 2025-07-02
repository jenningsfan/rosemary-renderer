use std::vec;

use crate::{types::{light::PointLight, sphere::Sphere, ray::Ray, colour::Colour,
    intersection::{Intersection, IntersectionComps}, material::Material}, Matrix, Tuple};

pub struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: Option<PointLight>) -> Self {
        Self { 
            objects,
            light,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        for obj in &self.objects {
            result.append(&mut obj.intersect(&ray));
        }

        result.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        result
    }

    pub fn shade_hit(&self, comps: &IntersectionComps) -> Colour {
        comps.obj.material.lighting(
            comps.over_point,
            &self.light.unwrap(),
            comps.eye,
            comps.normal,
            self.is_shadowed(comps.over_point)
        )
    }

    pub fn colour_at(&self, ray: &Ray) -> Colour {
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);

        if let Some(hit) = hit {
            let comps = hit.comps(ray);
            self.shade_hit(&comps)
        }
        else {
            Colour::black()
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        assert!(point.is_point());

        let vec_point_light = self.light.unwrap().pos - point;
        let distance = vec_point_light.magnitude();
        let direction = vec_point_light.norm();

        let ray = Ray::new(point, direction);
        let inters = self.intersect(&ray);
        let hit = Intersection::hit(&inters);

        hit.map_or(false, |hit| hit.t < distance)
    }
}

impl Default for World {
    fn default() -> Self {
        let l = PointLight::new(Colour::new(1.0, 1.0, 1.0),
            Tuple::point(-10.0, 10.0, -10.0));
        
        let mut s1 = Sphere::default();
        let mut mat = Material::default();
        mat.colour = Colour::new(0.8, 1.0, 0.6);
        mat.diffuse = 0.7;
        mat.specular = 0.2;
        s1.material = mat;

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        Self::new(vec![s1, s2], Some(l))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let inters = w.intersect(&r);
        
        assert_eq!(inters.len(), 4);
        assert_eq!(inters[0].t, 4.0);
        assert_eq!(inters[1].t, 4.5);
        assert_eq!(inters[2].t, 5.5);
        assert_eq!(inters[3].t, 6.0);
    }

    #[test]
    fn shade_hit_outside() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let inter = Intersection::new(4.0, shape);
        let comps = inter.comps(&r);
        let colour = w.shade_hit(&comps);
        assert_eq!(colour, Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_hit_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(Colour::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.25, 0.0)));

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let inter = Intersection::new(0.5, shape);
        let comps = inter.comps(&r);
        let colour = w.shade_hit(&comps);
        assert_eq!(colour, Colour::new(0.90489, 0.90498, 0.90498));
    }

    #[test]
    fn colour_at_miss() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.colour_at(&r);
        assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
    }
    
    #[test]
    fn colour_at_hit() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.colour_at(&r);
        assert_eq!(c, Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn colour_at_complex_hit() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.colour_at(&r);
        assert_eq!(c, w.objects[1].material.colour);
    }

    #[test]
    fn is_shadowed() {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false); // nothing colinear

        let p = Tuple::point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true); // far side

        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false); // between object behind light

        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false); //object behind point
    }

    #[test]
    fn shade_hit_shadow() {
        let s2 = Sphere::new(Matrix::translation(0.0, 0.0, 10.0), Material::default());
        let w = World::new(vec![Sphere::default(), s2], 
        Some(PointLight::new(Colour::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0))));
        let r = Ray::new(Tuple::point(0.0,0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let inter = Intersection::new(4.0, &s2);
        let comps = inter.comps(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Colour::new(0.1, 0.1, 0.1));
    }
}