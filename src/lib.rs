pub mod types;
pub use types::tuple::Tuple;
pub use types::matrix::Matrix;

const EPSILON: f32 = 0.0001;

fn eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug)]
pub struct Projectile {
    pub pos: Tuple,
    vel: Tuple,
}

impl Projectile {
    pub fn new(pos: Tuple, vel: Tuple) -> Self {
        Self {
            pos,
            vel
        }
    }
}

#[derive(Debug)]
pub struct Enviroment {
    gravity: Tuple,
    wind: Tuple,
}

impl Enviroment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self {
            gravity,
            wind
        }
    }
}

pub fn tick(env: &Enviroment, proj: &mut Projectile) {
    proj.pos += proj.vel;
    proj.vel += env.gravity + env.wind;
}