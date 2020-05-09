#[allow(dead_code)]
pub fn newton_raphson(x: f32, f: fn(f32) -> f32, df: fn(f32) -> f32, tol: f32) -> f32 {
    let mut y = x;
    for _i in 0..30 {
        let dx = -f(y) / df(y);
        y += dx;
        if dx.abs() < tol {
            break;
        }
    }
    y
}
