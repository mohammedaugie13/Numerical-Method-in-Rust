pub fn bisection(mut a: f32, mut b: f32, f: fn(f32) -> f32) -> f32 {
    if f(a) * f(b) >= 0.0 {
        println!("Your initialisation is wrong")
    }
    let mut c = a;
    while b - a >= 0.01 {
        c = (b + a) / 2.0;

        if f(c) == 0.0 {
            break;
        }
        if f(a) * f(c) < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }
    c
}
