pub fn riemann_integral(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
  let h = (b - a) / n as f64;
  let mut sum = 0.0;
  for i in 0..n {
    sum += f(a + i as f64 * h);
  }
  sum * h
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_close(x: f64, y: f64, eps: f64) {
    assert!((x - y).abs() < eps);
  }

  #[test]
  fn test_riemann_integral_1() {
    let f = |x: f64| x.powi(2);
    assert_eq!(riemann_integral(&f, 0.0, 3.14, 1), 0.0);
  }

  #[test]
  fn test_riemann_integral_2() {
    let f = |x: f64| x.powi(2);
    assert_close(riemann_integral(&f, 0.0, 1.0, 10), 0.285, 0.000001);
  }

  #[test]
  fn test_riemann_integral_3() {
    let f = |x: f64| 1.0 / x;
    assert_close(
      riemann_integral(&f, 1.0, 2.0, 10),
      0.7187714031754279,
      0.000001,
    );
  }
}
