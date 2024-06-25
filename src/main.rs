use std::{fs::File, io::Write};
use rosemary_renderer::{tick, types::{canvas::Canvas, colour::Colour}, Enviroment, Projectile, Tuple, Matrix};

fn main() {
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
