/**
    Computes `$\int_a^b f(x) dx$` with an adaptive Simpson's method,
    to the given precision and recursion depth.

    https://en.wikipedia.org/wiki/Adaptive_Simpson%27s_method
*/
pub fn integrate<F>(x0: f32, x1: f32, eps: f32, max_depth: usize, f: F) -> f32
where
    F: Fn(f32) -> f32,
{
    #[rustfmt::skip]
    fn recursive_integrate<FI>(f: &FI, a: f32, b: f32, c: f32, fa: f32, fb: f32, fc: f32, i: f32, eps: f32, depth: usize) -> f32
    where
        FI: Fn(f32) -> f32,
    {
        let d = 0.5 * (a + b);

        // serious numerical trouble: it won't converge
        if eps * 0.5 == eps || a == d {
            return i;
        }

        let e = 0.5 * (b + c);
        let fd = f(d);
        let fe = f(e);

        let h1 = 1.0 / 12.0 * (c - a); // combined h=(c-a)/2 and h*1/6
        let i0 = h1 * (fa + 4.0 * fd + fb);
        let i1 = h1 * (fb + 4.0 * fe + fc);
        let ip = i0 + i1;

        let di = ip - i;

        if depth == 0 || di.abs() < (15.0 * eps) {
            return ip + di / 15.0;
        }

        recursive_integrate(f, a, d, b, fa, fd, fb, i0, 0.5 * eps, depth - 1) +
        recursive_integrate(f, b, e, c, fb, fe, fc, i1, 0.5 * eps, depth - 1)
    }

    let a = x0;
    let c = x1;
    let h = c - a;

    if h == 0.0 {
        return 0.0;
    }

    let b = 0.5 * (x0 + x1);

    let fa = f(a);
    let fb = f(b);
    let fc = f(c);

    let i = (c - a) * (1.0 / 6.0) * (fa + 4.0 * fb + fc);

    recursive_integrate(&f, a, b, c, fa, fb, fc, i, eps, max_depth)
}
