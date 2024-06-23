mod tuple;

const EPSILON: f32 = 0.00001;

fn eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}