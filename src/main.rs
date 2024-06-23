use std::{fs::File, io::Write};
use rosemary_renderer::{tick, types::{canvas::Canvas, colour::Colour}, Enviroment, Projectile, Tuple};

fn main() {
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
    write!(file, "{}", canvas.to_ppm());
}
