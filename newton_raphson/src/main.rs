fn f(x: f32) -> f32 {
    x.powi(4) - 6.4 * x.powi(3) + 6.45 * x.powi(2) + 20.538 * x - 31.752
}

fn df(x: f32) -> f32 {
    4.0 * x.powi(3) - 19.2 * x.powi(2) + 12.9 * x + 20.538
}

fn newton_raphson(x: f32, tol: f32) -> (f32) {
    let mut y = x;
    let mut dx = -f(y) / df(y);
    for i in 0..30 {
        dx = -f(y) / df(y);
        y += dx;
        if dx.abs() < tol {
            break;
        }
    }
    (y)
}

fn main() {
    let tol = 1e-9;
    let result = newton_raphson(1.9, tol);
    println!("The Value is {:.1}", result);
}
