use std::vec;

use crate::{types::{light::PointLight, sphere::Sphere, ray::Ray, colour::Colour,
    intersection::Intersection, material::Material}, Matrix, Tuple};

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

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut result = Vec::new();

        for obj in &self.objects {
            result.append(&mut obj.intersect(ray));
        }

        result.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        result
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
    use crate::types::tuple;

    use super::*;

    #[test]
    fn intersect() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let inters = w.intersect(r);
        
        assert_eq!(inters.len(), 4);
        assert_eq!(inters[0].t, 4.0);
        assert_eq!(inters[1].t, 4.5);
        assert_eq!(inters[2].t, 5.5);
        assert_eq!(inters[3].t, 6.0);
    }
}