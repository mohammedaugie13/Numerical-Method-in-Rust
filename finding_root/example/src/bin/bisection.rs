use bisection::bisection;

fn f(x: f32) -> f32 {
    x.powi(3) - x.powi(2) + 2.0
}

fn main() {
    let root = bisection(-200.0, 300.0, f);
    println!("The value {}", root)
}
