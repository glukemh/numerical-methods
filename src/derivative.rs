// pub fn d_dx(f: &dyn Fn(f64) -> f64, x: f64) -> f64 {
//   (f(x + f64::EPSILON) - f(x)) / f64::EPSILON
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   fn assert_close(x: f64, y: f64, eps: f64) {
//     println!("left: {}", x);
//     println!("right: {}", y);
//     assert!((x - y).abs() < eps);
//   }

//   #[test]
//   fn test_d_dx_1() {
//     let f = |x: f64| x.powi(2);
//     assert_close(d_dx(&f, 0.0), 0.0, 1e-10);
//   }

//   #[test]
//   fn test_d_dx_2() {
//     let f = |x: f64| x.powi(2);
//     assert_close(d_dx(&f, 3.0), 6.0, 1e-10);
//   }

//   #[test]
//   fn test_d_dx_3() {
//     let f = |x: f64| 1.0 / x;
//     assert_close(d_dx(&f, 5.0), -0.04, 1e-10);
//   }
// }
