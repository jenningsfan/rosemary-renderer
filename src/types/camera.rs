use std::f32::consts::PI;
use crate::{types::{canvas::Canvas, colour::Colour, ray::Ray, world::World}, Matrix, Tuple};

#[derive(Debug, Clone)]
pub struct Camera {
    hsize: f32,
    vsize: f32,
    fov: f32,
    transform: Matrix,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn new(hsize: f32, vsize: f32, fov: f32, transform: Matrix) -> Camera {
        let (pixel_size, half_width, half_height) = Self::calculate_pixel_size(hsize, vsize, fov);
        Camera {
            hsize,
            vsize,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
        }
    }

    fn calculate_pixel_size(hsize: f32, vsize: f32, fov: f32) -> (f32, f32, f32) {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize / vsize;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        }
        else {
            (half_view * aspect, half_view)
        };

        ((half_width * 2.0) / hsize as f32, half_width, half_height)
    }

    fn ray_for_pixel(&self, x: f32, y: f32) -> Ray {
        let xoffset = (x + 0.5) * self.pixel_size;
        let yoffset = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv_transform = self.transform.inverse().unwrap();
        let pixel = inv_transform * Tuple::point(world_x, world_y, -1.0);
        let origin = inv_transform * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).norm();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);

        for y in 0..(self.vsize as usize) {
            for x in 0..(self.hsize as usize) {
                let ray = self.ray_for_pixel(x as f32, y as f32);
                let col = world.colour_at(&ray);
                image[(x, y)] = col
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use super::*;

    #[test]
    fn constructor() {
        let c = Camera::new(160.0, 120.0, PI / 2.0, Matrix::identity(4));
        assert_eq!(c.hsize, 160.0);
        assert_eq!(c.vsize, 120.0);
        assert_eq!(c.fov, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity(4));
    }

    #[test]
    fn pixel_size() {
        let c = Camera::new(200.0, 125.0, PI / 2.0, Matrix::identity(4));
        assert_eq!(c.pixel_size, 0.01);
        let c = Camera::new(125.0, 200.0, PI / 2.0, Matrix::identity(4));
        assert_eq!(c.pixel_size, 0.01);
        dbg!(c);
    }

    #[test]
    fn ray_for_pixel() {
        let c = Camera::new(201.0, 101.0, PI / 2.0, Matrix::identity(4));
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));

        let c = Camera::new(201.0, 101.0, PI / 2.0, Matrix::identity(4));
        let r = c.ray_for_pixel(0.0, 0.0);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));

        let c = Camera::new(201.0, 101.0, PI / 2.0, Matrix::translation(0.0, -2.0, 5.0).rotate_y(PI / 4.0));
        let r = c.ray_for_pixel(100.0, 50.0);
        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn render() {
        let w = World::default();
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let c = Camera::new(11.0, 11.0, PI / 2.0, Matrix::view_transform(from, to, up));
        let image = c.render(&w);
        assert_eq!(image[(5, 5)], Colour::new(0.38066, 0.47583, 0.28550));
    }
}