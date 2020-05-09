use newton_raphson::newton_raphson;
use plot::line_dash;

fn f(x: f32) -> f32 {
    x.powi(4) - 6.4 * x.powi(3) + 6.45 * x.powi(2) + 20.538 * x - 31.752
}

fn df(x: f32) -> f32 {
    4.0 * x.powi(3) - 19.2 * x.powi(2) + 12.9 * x + 20.538
}

fn result(x: Vec<f32>) -> Vec<f32> {
    let mut data = Vec::new();
    for i in 0..10 {
        data.push(f(x[i]));
    }
    data
}

fn main() {
    let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let tol = 1e-9;
    let result = result(data.clone());
    let (_root, _results) = newton_raphson(1.9, f, df, tol);
    line_dash(data.clone(), result, "", "Newton-Raphson");
}
