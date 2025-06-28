use std::{fs::File, io::Write};
use std::f32::consts::PI;
use rosemary_renderer::types::camera::Camera;
use rosemary_renderer::types::light::PointLight;
use rosemary_renderer::types::material::Material;
use rosemary_renderer::types::ray::Ray;
use rosemary_renderer::types::sphere::Sphere;
use rosemary_renderer::types::world::World;
use rosemary_renderer::{tick, types::{canvas::Canvas, colour::Colour, intersection::Intersection}, Enviroment, Projectile, Tuple, Matrix};

fn projectile_fun() {
// projectile fun things
    let mut proj = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 2.0, 0.0).norm() * 11.0
    );

    let env = Enviroment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0)
    );

    let mut canvas = Canvas::new(1500, 500);

    let mut total_ticks = 0;
    while proj.pos.y > 0.0 {
        tick(&env, &mut proj);
        let x = proj.pos.x as usize;
        let y = canvas.height - proj.pos.y as usize;
        if x < canvas.width && y < canvas.height {
            canvas[(x, y)] = Colour::new(1.0, 0.0, 0.0)
        }
    }

    let mut file = File::create("images/proj.ppm").unwrap();
    write!(file, "{}", canvas.to_ppm()).unwrap();
}

fn matrix_fun() {
    // matrix fun things
    let identity = Matrix::identity(4);
    dbg!(identity.inverse());

    let mat = Matrix::new_4x4([
        8.0, -5.0, 9.0, 2.0,
        7.0, 5.0, 6.0, 1.0,
        -6.0, 0.0, 9.0, 6.0,
        -3.0, 0.0, -9.0, -4.0
    ]);
    dbg!(mat * mat.inverse().unwrap());
    dbg!(mat.transpose().inverse());
    dbg!(mat.inverse().unwrap().transpose());

    let mut identity = Matrix::identity(4);
    identity[(2, 0)] = 2.0;
    let tup = Tuple::new(1.0, 2.0, 3.0, 4.0);
    dbg!(identity * tup);
}

fn clock_fun() {
    let angle = (2.0 * PI) / 12.0; // 2pi / 12;
    let mut canvas = Canvas::new(1000, 1000);

    for i in 0..12 {
        let transform = Matrix::rotation_z(angle * i as f32).scale(50.0, 50.0, 0.0).translate(500.0, 500.0, 0.0);
        let point = Tuple::point(0.0, 1.0, 0.0) * transform;

        let x = point.x as usize;
        let y = point.y as usize;
        if x < canvas.width && y < canvas.height {
            canvas[(x, y)] = Colour::new(1.0, 0.0, 0.0);
        }
    }

    let mut file = File::create("images/clock.ppm").unwrap();
    write!(file, "{}", canvas.to_ppm()).unwrap();
}

fn sphere_fun() {
    let size = 800;
    let half = (size / 2) as f32;
    let mut canvas = Canvas::new(size, size);

    let mut sphere = Sphere::default();
    sphere.material = Material::default();
    sphere.material.colour = Colour::new(1.0, 0.8, 0.8);
    sphere.set_transform(Matrix::shearing(1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    //sphere.set_transform(Matrix::rotation_y(90.0));

    let light = PointLight::new(Colour::new(0.9, 0.7, 0.4), Tuple::point(10.0, -10.0, -10.0));

    //sphere.transform = Matrix::scaling(2.0, 2.0, 2.0);
    let red = Colour::new(1.0, 0.0, 0.0);
    let ray_direction = Tuple::vector(0.0, 0.0, 1.0);
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let pixel_size = wall_size / size as f32;

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).norm());
            if let Some(hit) = Intersection::hit(sphere.intersect(&ray)) {
                let hit_point = ray.position(hit.t);
                let hit_norm = hit.obj.normal(hit_point);
                let eye = -ray.direction;
                let col = hit.obj.material.lighting(hit_point, &light, eye, hit_norm);

                canvas[(x, y)] = col;
            }
        }
    }

    let mut file = File::create("images/sphere.ppm").unwrap();
    write!(file, "{}", canvas.to_ppm()).unwrap();
}

fn world_render() {
    let mut floor = Sphere::new(Matrix::scaling(10.0, 0.01, 10.0), Material::default());
    floor.material.colour = Colour::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let left_wall = Sphere::new(
        Matrix::scaling(10.0, 0.01, 10.0).rotate_x(PI / 2.0)
        .rotate_y(- PI / 4.0).translate(0.0, 0.0, 5.0), floor.material);

    let right_wall = Sphere::new(
        Matrix::scaling(10.0, 0.01, 10.0).rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0).translate(0.0, 0.0, 5.0), floor.material);

    let middle = Sphere::new(Matrix::translation(-0.5, 1.0, 0.5), 
        Material { colour: Colour::new(0.1, 1.0, 0.5), ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200.0 });

    let right = Sphere::new(Matrix::translation(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5), 
        Material { colour: Colour::new(0.5, 1.0, 0.1), ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200.0 });
        
    let left = Sphere::new(Matrix::translation(-2.5, 0.33, -0.75).scale(0.33, 0.33, 0.33), 
        Material { colour: Colour::new(1.0, 0.8, 0.1), ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200.0 });

    let light = PointLight::new(Colour::new(1.0, 1.0, 1.0), Tuple::point(-10.0, 10.0, -10.0));
    let world = World::new(vec![floor, left_wall, right_wall, middle, right, left], Some(light));

    let cam = Camera::new(600.0, 300.0, PI / 3.0, 
        Matrix::view_transform(Tuple::point(0.0, 1.5, -10.0),
            Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)));
        
    let canvas = cam.render(&world);
    let mut file = File::create(format!("images/world.ppm")).unwrap();
    write!(file, "{}", canvas.to_ppm()).unwrap();
}

fn main() {
    // projectile_fun();
    // matrix_fun();
    // clock_fun();
    // sphere_fun();
    world_render();
}
