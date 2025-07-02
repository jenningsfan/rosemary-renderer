use crate::{types::colour::Colour, Tuple};

use super::light::PointLight;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn lighting(&self, pos: Tuple, light: &PointLight, eye: Tuple, norm: Tuple, shadow: bool) -> Colour {
        assert!(pos.is_point());
        assert!(eye.is_vector());
        assert!(norm.is_vector());

        // combine material + light colours
        let col = self.colour * light.intensity;
        let light_vec = (light.pos - pos).norm(); // direction to light source
        
        let ambient = col * self.ambient;
        let light_dot_norm = light_vec * norm ; // dot of light vec and norm is cos of their angles
        
        // neg means light behind surface as it is cos
        if light_dot_norm < 0.0 || shadow {
            // as light is behind, no specular or diffuse so only ambient does stuff
            return ambient;
        }

        let diffuse = col * self.diffuse * light_dot_norm;
        let mut specular = Colour::black();

        let reflect_vec = -light_vec.reflect(norm);
        let reflect_dot_eye = reflect_vec * eye; // same drill, cos of angles
        
        // if neg, then light reflects away from eye so no specular
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(self.shininess);
            specular = light.intensity * self.specular * factor;
        }

        // final result is a combination of the 3
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            colour: Colour::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use super::*;

    #[test]
    fn default() {
        let material = Material::default();
        assert_eq!(material.colour, Colour::new(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    fn lighting() {
        let material = Material::default();
        let pos = Tuple::point(0.0, 0.0, 0.0);
        let col = Colour::new(1.0, 1.0, 1.0);

        // Eye directly between light and surface
        // Full ambient, diffuse and specular (0.1 + 0.9 + 0.9 = 1.9)
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 0.0, -10.0));
        let result = material.lighting(pos, &light, eye, norm, false);
        assert_eq!(result, Colour::new(1.9, 1.9, 1.9));

        // Eye between light and surface at 45deg angle off norm
        // Full ambient, diffuse (0.1 + 0.9 = 1.0)
        // No specular
        let eye = Tuple::vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 0.0, -10.0));
        let result = material.lighting(pos, &light, eye, norm, false);
        assert_eq!(result, Colour::new(1.0, 1.0, 1.0));

        // Eye directly opposite surface with light at 45deg angle off norm
        // Full ambient, some diffuse (0.1 + 0.9 * sqrt(2)/2 = 0.7364)
        // No specular
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 10.0, -10.0));
        let result = material.lighting(pos, &light, eye, norm, false);
        assert_eq!(result, Colour::new(0.7364, 0.7364, 0.7364));

        // Light at 45deg angle off norm and eye directly in reflection path
        // Full ambient, full specular some diffuse (0.1 + 0.9 * sqrt(2)/2 + 0.9 = 1.6364)
        let eye = Tuple::vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 10.0, -10.0));
        let result = material.lighting(pos, &light, eye, norm, false);
        assert_eq!(result, Colour::new(1.6364, 1.6364, 1.6364));

        // Light behind surface
        // Only ambient (0.1 + 0 + 0 = 0.1)
        // No diffuse or specular
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 0.0, 10.0));
        let result = material.lighting(pos, &light, eye, norm, false);
        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));

        // In shadown
        // Only ambient
        // No diffuse or specular
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let norm = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(col, Tuple::point(0.0, 0.0, -10.0));
        let result = material.lighting(pos, &light, eye, norm, true);
        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
    } 
}