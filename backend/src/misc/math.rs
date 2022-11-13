
pub fn nth_root(n: f64, A: f64) -> f64 {
  let p: f64 = 1e-9_f64;
  let mut x0: f64 = A/n;

  loop {
    let mut x1: f64 = ((n-1.) * x0 + A / f64::powf(x0, n-1.)) / n;
    if x1-x0.abs() < (x0*p).abs() { return x1; }
    x0 = x1;
  }
}
