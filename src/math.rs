pub fn integrate<F>(a: f32, b: f32, n: usize, f: F) -> f32
where
    F: Fn(f32) -> f32,
{
    let width = (b - a) / n as f32;

    let mut integral = 0.0;

    for step in 0..n {
        let step = step as f32;

        let x1 = a + step * width;
        let x2 = a + (step + 1.0) * width;

        integral += (x2 - x1) / 6.0 * (f(x1) + 4.0 * f(0.5 * (x1 + x2)) + f(x2));
    }

    integral
}
