use crate::floats::FloatOps;
const SQRT_EPSILON: f64 = 0.000000014901161193847656;

pub fn d_dx(f: &dyn Fn(f64) -> f64, x: f64) -> f64 {
    let h = if x == 0.0 {
        f64::EPSILON
    } else {
        SQRT_EPSILON * x
    };
    (f(x + h) - f(x)) / h
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, eps: f64) {
        println!("left: {}", a);
        println!("right: {}", b);
        assert!((a - b).abs() < eps);
    }

    #[test]
    fn test_d_dx_1() {
        let f = |x: f64| x.powi(2);
        let x = 0.0;
        assert_close(d_dx(&f, x), 0.0, 1e-7);
    }

    #[test]
    fn test_d_dx_2() {
        let f = |x: f64| x.powi(2);
        let x = 3.0;
        assert_close(d_dx(&f, x), 6.0, 1e-7);
    }

    #[test]
    fn test_d_dx_3() {
        let f = |x: f64| 1.0 / x;
        let x = 5.0;
        assert_close(d_dx(&f, x), -0.04, 1e-6);
    }
}
