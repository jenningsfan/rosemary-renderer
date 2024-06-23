use rosemary_renderer::{Projectile, Enviroment, Tuple, tick};

fn main() {
    let mut proj = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.0, 0.0).norm() 
    );

    let env = Enviroment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0)
    );

    let mut total_ticks = 0;
    while proj.pos.y > 0.0 {
        tick(&env, &mut proj);
        total_ticks += 1;
        dbg!(&proj.pos);
    }
    
    dbg!(total_ticks);
}
